use salvo::prelude::*;

pub fn routes() -> Router {
    Router::new()
        .path("/auth")
        .push(Router::new().path("/signup").post(signup))
        .push(Router::new().path("/signin").post(signin))
}

#[handler]
fn signup() {
    todo!()
}

#[handler]
fn signin() {
    todo!()
}
