mod migration_0001_user;

use std::future::Future;
use std::pin::Pin;

// use sea_query::{MysqlQueryBuilder, PostgresQueryBuilder, SchemaBuilder, SqliteQueryBuilder};

use sqlx::{
    error::BoxDynError,
    migrate::{Migration, MigrationSource, Migrator},
    Any, Pool,
};

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[derive(Debug)]
struct AppMigrations(Vec<Migration>);

impl AppMigrations {
    pub fn new(pool: Pool<Any>) -> anyhow::Result<Self> {
        // let schema_builder = match pool.any_kind() {
        //     AnyKind::Postgres => &PostgresQueryBuilder,
        //     AnyKind::MySql => &MysqlQueryBuilder,
        //     AnyKind::Sqlite => &SqliteQueryBuilder,
        // };
        let schema_builder = &sea_query::SqliteQueryBuilder;

        Ok(AppMigrations(vec![migration_0001_user::up(
            pool.clone(),
            schema_builder,
        )?]))
    }
}

impl MigrationSource<'static> for AppMigrations {
    fn resolve(self) -> BoxFuture<'static, std::result::Result<Vec<Migration>, BoxDynError>> {
        Box::pin(async move { Ok(self.0) })
    }
}

pub async fn migrate_up(pool: Pool<Any>) -> anyhow::Result<()> {
    let migrations = AppMigrations::new(pool.clone())?;
    let migrator = Migrator::new(migrations).await?;
    migrator.run(&pool).await?;
    Ok(())
}

pub async fn migrate_down(pool: Pool<Any>, target: i64) -> anyhow::Result<()> {
    let migrations = AppMigrations::new(pool.clone())?;
    let migrator = Migrator::new(migrations).await?;
    migrator.undo(&pool, target).await?;
    Ok(())
}
