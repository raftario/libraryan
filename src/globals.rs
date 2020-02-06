use crate::config::Config;
use diesel::{
    r2d2::{self, ConnectionManager},
    sqlite::SqliteConnection,
};
use lazy_static::lazy_static;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

lazy_static! {
    pub static ref CONFIG: Config = envy::from_env().expect("Invalid config");
    pub static ref POOL: Pool = {
        let manager = ConnectionManager::<SqliteConnection>::new(&CONFIG.database_url);
        r2d2::Pool::builder()
            .max_size(CONFIG.database_pool_size)
            .build(manager)
            .expect("Can't create pool")
    };
}
