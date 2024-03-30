use salvo::prelude::*;

const STYLES_CSS: &str = grass::include!("./crates/server/assets/stylesheets/styles.scss");

pub fn routes() -> Router {
    Router::new().path("assets/styles.css").get(get_stylesheet)
}

#[handler]
fn get_stylesheet(res: &mut Response) {
    // TODO: handle release builds with hashed filenames and cache
    res.render(Text::Css(STYLES_CSS))
}

pub fn styles_css_href<'a>() -> &'a str {
    // TODO: handle release builds with hashed filenames and cache
    "/assets/styles.css"
}
