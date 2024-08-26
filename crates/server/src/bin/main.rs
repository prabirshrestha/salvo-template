use server::cli::Cli;
use sqlx::any::install_default_drivers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    install_default_drivers();

    Cli::from_env().run().await?;

    Ok(())
}
