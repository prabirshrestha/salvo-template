use server::{app::App, router, AppConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let app_config = AppConfig::load()?;
    let app = App::new(app_config)?;
    app.serve(router()).await?;

    Ok(())
}
