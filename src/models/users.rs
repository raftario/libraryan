use crate::models::permissions::Permission;
use chrono::NaiveDateTime;
use diesel::QueryResult;
use std::collections::HashSet;

pub struct User {
    pub id: i32,
    pub login: String,
    pub display_name: String,
    pub permissions: HashSet<Permission>,
    pub registered: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub last_login: Option<NaiveDateTime>,
}

impl User {
    pub fn by_id(id: i32) -> QueryResult<Self> {
        unimplemented!()
    }
}

mod db {
    use crate::schema::{user_permission_mappings, users};
    use chrono::NaiveDateTime;

    #[derive(Identifiable, Queryable, PartialEq, Debug)]
    #[table_name = "users"]
    pub struct User {
        pub id: i32,
        pub login: String,
        pub password: Vec<u8>,
        pub display_name: String,
        pub registered: NaiveDateTime,
        pub updated: NaiveDateTime,
        pub last_login: Option<NaiveDateTime>,
    }

    #[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
    #[table_name = "user_permission_mappings"]
    #[belongs_to(User)]
    pub struct UserPermissionMapping {
        pub id: i32,
        pub user_id: i32,
        pub permission_id: i32,
    }
}
