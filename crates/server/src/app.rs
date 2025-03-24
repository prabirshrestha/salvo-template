use std::{pin::Pin, sync::Arc};

use crate::{
    AppResult,
    app_config::AppConfig,
    migrations::migrate_up,
    services::{backend::user::SurrealUserService, user::UserService},
    web_server::run_web_server,
    worker::run_worker,
};
use futures_util::future::join_all;
use salvo::prelude::*;
use tokio::signal;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

#[derive(Clone)]
pub struct App {
    pub app_config: Arc<AppConfig>,
    pub db: Arc<surrealdb::Surreal<surrealdb::engine::any::Any>>,

    pub user_service: Arc<dyn UserService>,
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

    pub async fn new_from_env() -> AppResult<Self> {
        let app_config = Arc::new(AppConfig::from_path(None)?);
        Self::new_from_config(app_config).await
    }

    pub async fn new_from_config(app_config: Arc<AppConfig>) -> AppResult<Self> {
        let mut surrealdb_config = surrealdb::opt::Config::new()
            .set_strict(true)
            .capabilities(surrealdb::opt::capabilities::Capabilities::all());

        if app_config.database.username.is_some() && app_config.database.password.is_some() {
            surrealdb_config = surrealdb_config.user(surrealdb::opt::auth::Root {
                username: &app_config.database.username.clone().unwrap(),
                password: &app_config.database.password.clone().unwrap(),
            });
        }

        let db = Arc::new(
            surrealdb::engine::any::connect((&app_config.database.url, surrealdb_config)).await?,
        );

        if app_config.database.auto_migrate {
            migrate_up(db.clone(), app_config.clone()).await?;
        }

        let user_service = Arc::new(SurrealUserService::new(db.clone()));

        let app = Self {
            app_config,
            db,
            user_service,
        };

        Ok(app)
    }

    pub fn app_config(&self) -> &AppConfig {
        &self.app_config
    }

    pub async fn run(self) -> AppResult<()> {
        if self.app_config.web.enabled || self.app_config.worker.enabled {
            let mut futures: Vec<Pin<Box<dyn Future<Output = AppResult<()>>>>> = Vec::new();

            let cancel_token = CancellationToken::new();

            let cancel_handle = tokio::spawn({
                let cancel_token = cancel_token.clone();
                async move {
                    wait_for_signal().await;
                    info!("Gracefully shutting down...");
                    cancel_token.cancel();
                }
            });

            if self.app_config.worker.enabled {
                let worker = run_worker(self.clone(), cancel_token.clone());
                futures.push(Box::pin(worker));
            }

            if self.app_config.web.enabled {
                let web_server = run_web_server(self.clone(), cancel_token.clone());
                futures.push(Box::pin(web_server));
            }

            let results = join_all(futures).await;
            let mut error = None;
            for result in results {
                if let Err(e) = result {
                    error!("Future failed with error: {:?}", e);
                    error = Some(e);
                }
            }

            cancel_handle.await?;

            if let Some(e) = error {
                return Err(e);
            }
        } else {
            error!("No web or worker enabled");
        }

        Ok(())
    }
}

async fn wait_for_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Ctrl+C signal received"),
        _ = terminate => info!("SIGTERM signal received"),
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
