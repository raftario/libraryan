use crate::{error::Error, globals::POOL, schema};
pub use db::{Author, AuthorSummary};
use diesel::prelude::*;

impl AuthorSummary {
    pub fn by_id(id: i32) -> Result<Self, Error> {
        let connection = POOL.get()?;
        Ok(schema::authors::dsl::authors
            .find(id)
            .select((schema::authors::id, schema::authors::name))
            .first::<Self>(&connection)?)
    }
}

mod db {
    use crate::schema::authors;
    use chrono::{NaiveDate, NaiveDateTime};

    #[derive(Identifiable, Queryable, Clone)]
    #[table_name = "authors"]
    pub struct Author {
        pub id: i32,
        pub name: String,
        pub description: Option<String>,
        pub born: Option<NaiveDate>,
        pub image: Option<Vec<u8>>,
        pub created: NaiveDateTime,
        pub creator: i32,
        pub updated: NaiveDateTime,
    }

    #[derive(Identifiable, Queryable, Clone)]
    #[table_name = "authors"]
    pub struct AuthorSummary {
        pub id: i32,
        pub name: String,
    }
}
