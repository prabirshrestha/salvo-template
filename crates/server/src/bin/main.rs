use server::{app::App, router};
use sqlx::any::install_default_drivers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    install_default_drivers();

    let app = App::new_from_env().await?;
    app.serve(router()).await?;

    Ok(())
}
