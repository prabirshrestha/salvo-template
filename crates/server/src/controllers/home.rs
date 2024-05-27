use anyhow::Result;
pub use salvo::prelude::*;

use crate::{
    app::{App, AppDepot},
    templates,
    utils::render::RenderExt,
};

pub fn routes() -> Router {
    Router::new().get(home)
}

#[handler]
async fn home(res: &mut Response, depot: &Depot) -> Result<()> {
    let App {
        user_service: _user_service,
        ..
    } = depot.app();

    res.render_html(|o| templates::home::home_html(o))?;

    Ok(())
}
