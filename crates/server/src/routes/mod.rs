pub use salvo::prelude::*;
mod api;
pub mod assets;
mod home;

pub fn router() -> Router {
    Router::new()
        .push(home::routes())
        .push(assets::routes())
        .push(api::routes())
}
