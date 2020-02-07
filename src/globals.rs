use crate::{
    config::Config,
    models::{books::Book, users::User},
};
use diesel::{
    r2d2::{self, ConnectionManager},
    sqlite::SqliteConnection,
};
use lazy_static::lazy_static;
use lru::LruCache;
use std::sync::Mutex;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub type UserCache = LruCache<i32, User>;
pub type BookCache = LruCache<i32, Book>;

lazy_static! {
    pub static ref CONFIG: Config = envy::from_env().expect("Invalid config");
    pub static ref POOL: Pool = {
        let manager = ConnectionManager::<SqliteConnection>::new(&CONFIG.database_url);
        r2d2::Pool::builder()
            .max_size(CONFIG.database_pool_size)
            .build(manager)
            .expect("Can't create pool")
    };
    pub static ref USER_CACHE: Mutex<UserCache> =
        Mutex::new(UserCache::new(CONFIG.cache.user_cache_size));
    pub static ref BOOK_CACHE: Mutex<BookCache> =
        Mutex::new(BookCache::new(CONFIG.cache.book_cache_size));
}
