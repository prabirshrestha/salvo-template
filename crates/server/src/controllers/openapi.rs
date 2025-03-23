use salvo::prelude::*;

pub fn with_openapi(router: Router) -> Router {
    let doc = OpenApi::new("objstor", "0.0.1").merge_router(&router);

    router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(Scalar::new("/api-doc/openapi.json").into_router("/api-doc"))
}
