use salvo::{oapi, prelude::*};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ConfigError(#[from] schematic::ConfigError),
    #[error(transparent)]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    ValidateError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    SalvoError(#[from] salvo::Error),
    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),
    #[error(transparent)]
    MqError(#[from] mq::Error),
    #[error(transparent)]
    SurrealdbError(#[from] surrealdb::Error),
    #[error(transparent)]
    SurrealdbMigratorError(#[from] surrealdb_migrator::Error),
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),
    #[error("OtherError: {0}")]
    OtherError(Box<dyn std::error::Error + Send + Sync + 'static>),
}

#[derive(Debug, Serialize)]
pub struct JsonError {
    pub code: String,
    pub message: String,
}

#[async_trait]
impl Writer for AppError {
    async fn write(mut self, req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        error!("Internal Server Error: {:?}", self);

        let is_json = req
            .first_accept()
            .map(|c| c.subtype() == mime::JSON)
            .unwrap_or(false);

        match self {
            _ => {
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                if is_json {
                    res.render(Json(JsonError {
                        code: "InternalServerError".to_string(),
                        message: "Internal Server Error".to_string(),
                    }));
                } else {
                    res.render("Internal Server Error");
                }
            }
        }
    }
}

impl EndpointOutRegister for AppError {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation.responses.insert(
            StatusCode::INTERNAL_SERVER_ERROR.as_str(),
            oapi::Response::new("Internal Server Error")
                .add_content("application/json", to_schema(components)),
        );
    }
}

fn to_schema(components: &mut oapi::Components) -> oapi::RefOr<oapi::schema::Schema> {
    let symbol = "error".to_string(); // std::any::type_name::<AppError>().replace("::", ".");
    let schema = oapi::Schema::from(
        oapi::Object::new()
            .property(
                "error",
                oapi::Object::new()
                    .property("code", String::to_schema(components))
                    .required("code")
                    .property("message", String::to_schema(components))
                    .required("message"),
            )
            .required("error"),
    );
    components.schemas.insert(symbol.clone(), schema);
    oapi::RefOr::Ref(oapi::Ref::new(format!("#/components/schemas/{}", symbol)))
}
