use server::app::App;
use sqlx::any::install_default_drivers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    install_default_drivers();

    App::new_from_env().await?.serve().await?;

    Ok(())
}
