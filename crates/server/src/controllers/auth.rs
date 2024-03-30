use salvo::prelude::*;

pub fn routes() -> Router {
    Router::new()
        .push(
            Router::new()
                .path("/signup")
                .get(get_signup)
                .post(post_signup),
        )
        .push(
            Router::new()
                .path("/signin")
                .get(get_signin)
                .post(post_signin),
        )
}

#[handler]
fn get_signup() {
    todo!()
}

#[handler]
fn post_signup() {
    todo!()
}

#[handler]
fn get_signin() {
    todo!()
}

#[handler]
fn post_signin() {
    todo!()
}
