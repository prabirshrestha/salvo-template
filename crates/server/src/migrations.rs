use std::sync::Arc;

use surrealdb::{Surreal, engine::any::Any};
use surrealdb_migrator::Migrations;

use include_dir::{Dir, include_dir};

use crate::{app_config::AppConfig, error::AppResult};
static MIGRATION_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");

pub async fn migrate_up(db: Arc<Surreal<Any>>, config: Arc<AppConfig>) -> AppResult<()> {
    if config.database.define_ns {
        db.query(format!("DEFINE NAMESPACE {};", &config.database.ns))
            .await?;
    }

    if config.database.define_db {
        db.query(format!(
            "USE NAMESPACE {}; DEFINE DATABASE {};",
            &config.database.ns, &config.database.db
        ))
        .await?;
    }

    db.use_ns(&config.database.ns)
        .use_db(&config.database.db)
        .await?;

    let migrations = Migrations::from_directory(&MIGRATION_DIR)?;
    migrations.to_latest(&db).await?;

    Ok(())
}
