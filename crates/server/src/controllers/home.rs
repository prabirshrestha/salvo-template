pub use salvo::prelude::*;

use crate::{
    app::{App, AppDepot},
    views::layout::Layout,
};

pub fn routes() -> Router {
    Router::new().get(home)
}

markup::define! {
    HomeView {
        h1 { "Hello World" }
    }
}

#[handler]
async fn home(res: &mut Response, depot: &Depot) {
    let App { user_service, .. } = depot.app();
    // TODO: do something with user_service
    res.render(Text::Html(Layout { main: HomeView {} }.to_string()))
}
