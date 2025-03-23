use salvo::{oapi::extract::JsonBody, prelude::*};

use crate::{
    AppResult,
    app::{App, AppDepot},
    services::user::{CreateUserRequest, CreateUserResponse},
};

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

#[endpoint(tags("auth"))]
async fn post_signup(
    body: JsonBody<CreateUserRequest>,
    depo: &mut Depot,
) -> AppResult<Json<CreateUserResponse>> {
    let App { user_service, .. } = depo.app();

    let new_user = user_service.create_user(body.into_inner()).await?;

    Ok(Json(new_user))
}

#[handler]
fn get_signin() {
    todo!()
}

#[handler]
fn post_signin() {
    todo!()
}
