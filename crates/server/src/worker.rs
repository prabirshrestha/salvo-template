use mq::{Consumer, Worker};
use mq_surreal::SurrealJobProcessor;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::{AppError, AppResult, app::App};

pub async fn run_worker(app: App, cancel_token: CancellationToken) -> AppResult<()> {
    let task = tokio::spawn(async move {
        info!("Starting worker...");
        Worker::new(Consumer::new())
            .with_poll_interval(Some(3000))
            .with_cancellation_token(cancel_token)
            .run(SurrealJobProcessor::new(app.db.clone(), "queue"))
            .await?;
        info!("Worker stopped");
        Ok(()) as AppResult<()>
    });

    let _ = task.await.map_err(|e| {
        error!("awaiting run_worker {}", e.to_string());
        AppError::OtherError(Box::new(e))
    })?;

    Ok(())
}
