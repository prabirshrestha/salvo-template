use server::cli::Cli;
use sqlx::any::install_default_drivers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    install_default_drivers();

    Cli::from_env().run().await?;

    Ok(())
}
