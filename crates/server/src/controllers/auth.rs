use salvo::{oapi::extract::JsonBody, prelude::*};

use crate::{
    AppResult,
    app::{App, AppDepot},
    services::user::{SignUpRequest, SignUpResponse},
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
    body: JsonBody<SignUpRequest>,
    depo: &mut Depot,
) -> AppResult<Json<SignUpResponse>> {
    let App { user_service, .. } = depo.app();

    let new_user = user_service.signup(&body).await?;

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
