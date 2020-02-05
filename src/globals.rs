use crate::config::Config;
use diesel::{
    r2d2::{self, ConnectionManager},
    sqlite::SqliteConnection,
};
use lazy_static::lazy_static;
use std::process;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

lazy_static! {
    pub static ref CONFIG: Config = match envy::from_env() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Invalid config: {}", e);
            process::exit(1);
        }
    };
    pub static ref POOL: Pool = unimplemented!();
}
