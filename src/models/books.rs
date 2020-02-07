use crate::{
    error::Error,
    globals::{BOOK_CACHE, POOL},
    models::users::User,
    schema,
};
pub use db::Book;
use diesel::prelude::*;

impl Book {
    pub fn by_id(id: i32) -> Result<Self, Error> {
        let mut cache = BOOK_CACHE.lock().expect("Can't lock mutex");
        let book = cache.get(&id);
        if let Some(b) = book {
            return Ok((*b).clone());
        }

        let connection = POOL.get().unwrap();

        let book: Book = schema::books::dsl::books
            .find(id)
            .first::<Self>(&connection)?;
        cache.put(book.id, book.clone());
        Ok(book)
    }

    pub fn uploader(&self) -> Result<User, Error> {
        User::by_id(self.uploader)
    }
}

mod db {
    use crate::schema::books;
    use chrono::{NaiveDate, NaiveDateTime};

    #[derive(Identifiable, Queryable, PartialEq, Clone, Debug)]
    #[table_name = "books"]
    pub struct Book {
        pub id: i32,
        pub title: String,
        pub subtitle: Option<String>,
        pub description: Option<String>,
        pub isbn: Option<i64>,
        pub published: Option<NaiveDate>,
        pub publisher: Option<String>,
        pub page_count: Option<i16>,
        pub image: Option<Vec<u8>>,
        pub uploaded: NaiveDateTime,
        pub uploader: i32,
        pub updated: NaiveDateTime,
    }
}
