use crate::{migrations::migrate_up, AppConfig};
use salvo::{prelude::*, server::ServerHandle};
use sqlx::{Any, Pool};
use tokio::signal;
use tracing::info;

pub struct App {
    app_config: AppConfig,
    db: Pool<Any>,
}

impl App {
    pub async fn new_from_env() -> anyhow::Result<Self> {
        let app_config = AppConfig::load()?;
        Self::new_from_config(app_config).await
    }

    pub async fn new_from_config(app_config: AppConfig) -> anyhow::Result<Self> {
        let db = Pool::connect(&app_config.database).await?;
        let app = Self { app_config, db };
        app.init().await?;
        Ok(app)
    }

    async fn init(&self) -> anyhow::Result<()> {
        if self.app_config.auto_migrate {
            migrate_up(self.db.clone()).await?;
        }

        Ok(())
    }

    pub fn app_config(&self) -> &AppConfig {
        &self.app_config
    }

    pub async fn serve<S>(self, service: S) -> anyhow::Result<()>
    where
        S: Into<Service> + Send,
    {
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

        tokio::spawn(shutdown_signal(handle));

        server.serve(service).await;

        Ok(())
    }
}

async fn shutdown_signal(handle: ServerHandle) {
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
}
