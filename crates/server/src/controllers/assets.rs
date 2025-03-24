pub use salvo::prelude::*;

use crate::{AppResult, templates::statics::StaticFile};

pub fn routes() -> Router {
    Router::with_path("/assets/<name>").get(get_assets)
}

#[handler]
fn get_assets(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let name = req.param("name").unwrap();
    let data = StaticFile::get(name).unwrap();
    res.add_header(
        salvo::http::header::CONTENT_TYPE,
        data.mime.to_string(),
        true,
    )?
    .add_header(
        salvo::http::header::CACHE_CONTROL,
        "max-age=31536000", // 1 year as second
        true,
    )?
    .write_body(data.content)?;
    Ok(())
}
