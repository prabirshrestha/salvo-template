use sea_query::{ColumnDef, SchemaBuilder, Table};
use sqlx::{
    migrate::{Migration, MigrationType},
    AnyPool, Error,
};

use crate::entities::user::UserIden;

pub fn up(_pool: AnyPool, schema_builder: &dyn SchemaBuilder) -> Result<Migration, Error> {
    let sql = Table::create()
        .table(UserIden::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(UserIden::Id)
                .string()
                .string_len(255)
                .not_null()
                .primary_key(),
        )
        .col(
            ColumnDef::new(UserIden::Username)
                .string()
                .string_len(255)
                .not_null()
                .unique_key(),
        )
        .col(
            ColumnDef::new(UserIden::PasswordHash)
                .string()
                .string_len(255)
                .not_null(),
        )
        .col(ColumnDef::new(UserIden::IsLocked).boolean().not_null())
        .col(ColumnDef::new(UserIden::IsAdmin).boolean().not_null())
        .col(
            ColumnDef::new(UserIden::ColorScheme)
                .string()
                .string_len(255)
                .not_null(),
        )
        .build_any(schema_builder);

    Ok(Migration::new(
        1,
        "create user".into(),
        MigrationType::Simple,
        sql.into(),
    ))
}
