pub use salvo::prelude::*;

pub mod errors;

mod assets;
mod auth;
mod home;
mod openapi;

pub fn router() -> Router {
    let router = Router::new()
        .push(home::routes())
        .push(assets::routes())
        .push(auth::routes());

    openapi::with_openapi(router)
}
