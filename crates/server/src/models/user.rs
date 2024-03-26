use sea_query::Iden;
use secrecy::Secret;

#[derive(Iden)]
#[iden = "user"]
pub enum UserIden {
    Table,
    Id,
    Username,
    PasswordHash,
    IsLocked,
    IsAdmin,
    ColorScheme,
}

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: Secret<String>,
    pub is_locked: bool,
    pub is_admin: bool,
    pub color_scheme: String,
}
