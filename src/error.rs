#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database error")]
    Db(#[from] diesel::result::Error),
}
