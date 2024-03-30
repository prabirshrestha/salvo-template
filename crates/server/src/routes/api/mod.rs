use salvo::prelude::*;

mod auth;

pub fn routes() -> Router {
    Router::new().path("/api").push(auth::routes())
}
