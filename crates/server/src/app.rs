use std::sync::Arc;

use crate::{
    app_config::AppConfig,
    controllers,
    migrations::migrate_up,
    services::{self, backend::user::SqlUserService},
};
use salvo::{prelude::*, server::ServerHandle};
use sqlx::{Any, Pool};
use tokio::signal;
use tracing::{error, info};

#[derive(Clone)]
pub struct App {
    postgresql: Option<postgresql_embedded::PostgreSQL>,
    pub db: Pool<Any>,
    pub app_config: Arc<AppConfig>,
    pub user_service: Arc<dyn services::user::UserService>,
}

impl App {
    pub fn version() -> String {
        format!(
            "{} {}-{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            &env!("VERGEN_GIT_SHA")[..9]
        )
    }

    pub async fn new_from_env() -> anyhow::Result<Self> {
        let app_config = Arc::new(AppConfig::from_path()?);
        Self::new_from_config(app_config).await
    }

    pub async fn new_from_config(app_config: Arc<AppConfig>) -> anyhow::Result<Self> {
        let (postgresql, database_url) = if app_config.database.use_embedded {
            info!("Setting up embedded PostgreSQL");

            let settings = postgresql_embedded::Settings::from_url(&app_config.database.url)?;

            let mut postgresql = postgresql_embedded::PostgreSQL::new(settings);
            postgresql.setup().await?;
            info!("Starting embedded PostgreSQL");
            postgresql.start().await?;

            if !postgresql
                .database_exists(&app_config.database.db_name)
                .await?
            {
                info!(
                    "Creating embedded database {}",
                    &app_config.database.db_name
                );
                postgresql
                    .create_database(&app_config.database.db_name)
                    .await?;
                info!("Created embedded database {}", &app_config.database.db_name);
            }

            let settings = postgresql.settings().clone();
            (
                Some(postgresql),
                &settings.url(&app_config.database.db_name),
            )
        } else {
            info!("Using external database");
            (None, &app_config.database.url)
        };

        info!("Connecting to database");
        let db = Pool::connect(database_url).await?;
        info!("Connected to the database");

        if app_config.auto_migrate {
            info!("Running auto migrations");
            migrate_up(db.clone()).await?;
        }

        let user_service = Arc::new(SqlUserService::new(db.clone()));

        let app = Self {
            postgresql,
            db,
            app_config,
            user_service,
        };

        Ok(app)
    }

    pub fn app_config(&self) -> &AppConfig {
        &self.app_config
    }

    pub async fn run(self) -> anyhow::Result<()> {
        info!("Starting app");

        info!("Starting server");

        let acceptor = TcpListener::new(format!(
            "{}:{}",
            &self.app_config().host,
            &self.app_config().port
        ))
        .bind()
        .await;

        let server = Server::new(acceptor);
        let handle = server.handle();

        tokio::spawn(shutdown_signal(handle, self.postgresql.clone()));

        let service = Router::new()
            .hoop(salvo::affix_state::inject(self.clone()))
            .push(controllers::router());

        server.serve(service).await;

        Ok(())
    }
}

async fn shutdown_signal(
    handle: ServerHandle,
    postgresql: Option<postgresql_embedded::PostgreSQL>,
) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("ctrl_c signal received"),
        _ = terminate => info!("terminate signal received"),
    }

    handle.stop_graceful(std::time::Duration::from_secs(60));

    if let Some(postgresql) = postgresql {
        info!("Stopping embedded PostgreSQL");
        if let Err(e) = postgresql.stop().await {
            error!("Failed to stop embedded PostgreSQL: {:?}", e);
        } else {
            info!("Stopped embedded PostgreSQL");
        }
    }
}

pub trait AppDepot {
    fn app(&self) -> &App;
}

impl AppDepot for Depot {
    fn app(&self) -> &App {
        self.obtain::<App>().unwrap()
    }
}
