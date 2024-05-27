pub use salvo::prelude::*;

mod assets;
mod auth;
mod home;

pub fn router() -> Router {
    Router::new()
        .push(home::routes())
        .push(assets::routes())
        .push(auth::routes())
}
