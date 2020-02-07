#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database error")]
    Db(#[from] diesel::result::Error),
    #[error("Argon2 error")]
    Argon2(#[from] argon2::Error),
    #[error("Database pool error")]
    DbPool(#[from] r2d2::Error),
}
