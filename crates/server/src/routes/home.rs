pub use salvo::prelude::*;

use crate::views::layout::Layout;

pub fn routes() -> Router {
    Router::new().get(home)
}

markup::define! {
    HomeView {
        h1 { "Hello World" }
    }
}

#[handler]
async fn home(res: &mut Response) {
    res.render(Text::Html(Layout { main: HomeView {} }.to_string()))
}
