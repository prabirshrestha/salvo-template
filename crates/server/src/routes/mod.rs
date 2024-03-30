pub use salvo::prelude::*;
pub mod assets;
mod home;

pub fn router() -> Router {
    Router::new().push(home::routes()).push(assets::router())
}
