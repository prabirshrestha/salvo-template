use anyhow::Result;
pub use salvo::prelude::*;

use crate::{
    app::{App, AppDepot},
    templates,
    utils::render::render_html,
};

pub fn routes() -> Router {
    Router::new().get(home)
}

#[handler]
async fn home(res: &mut Response, depot: &Depot) -> Result<()> {
    let App { user_service, .. } = depot.app();

    render_html(res, |o| templates::home::home_html(o))?;

    Ok(())
}
