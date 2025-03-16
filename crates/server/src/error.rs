use salvo::prelude::*;

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
    SalvoError(#[from] salvo::Error),
    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),
    #[error(transparent)]
    MqError(#[from] mq::Error),
    #[error(transparent)]
    SurrealdbError(#[from] surrealdb::Error),
    #[error(transparent)]
    SurrealdbMigratorError(#[from] surrealdb_migrator::Error),
    #[error("OtherError: {0}")]
    OtherError(Box<dyn std::error::Error + Send + Sync + 'static>),
}

#[async_trait]
impl salvo::Writer for AppError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.render("Internal Server Error");
    }
}
