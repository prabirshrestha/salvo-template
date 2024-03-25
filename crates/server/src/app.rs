use crate::AppConfig;
use salvo::{prelude::*, server::ServerHandle};
use tokio::signal;
use tracing::info;

pub struct App {
    app_config: AppConfig,
}

impl App {
    pub fn new(app_config: AppConfig) -> anyhow::Result<Self> {
        Ok(Self { app_config })
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
