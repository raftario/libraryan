#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database error")]
    Db(#[from] diesel::result::Error),
    #[error("Argon2 error")]
    Argon2(#[from] argon2::Error),
}
