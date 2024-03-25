pub use salvo::prelude::*;

pub fn routes() -> Router {
    Router::new().get(home)
}

#[handler]
async fn home() -> &'static str {
    "Hello World"
}
