use crate::{globals::POOL, models::permissions::Permission};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use std::{collections::HashSet, convert::TryFrom};

pub struct User {
    pub id: i32,
    pub login: String,
    pub display_name: String,
    pub permissions: HashSet<Permission>,
    pub registered: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub last_login: Option<NaiveDateTime>,
}

impl TryFrom<db::User> for User {
    type Error = diesel::result::Error;

    fn try_from(value: db::User) -> Result<Self, Self::Error> {
        let connection = POOL.get().unwrap();

        let permission_mappings = db::UserPermissionMapping::belonging_to(&value)
            .load::<db::UserPermissionMapping>(&connection)?;

        let mut permissions = HashSet::with_capacity(permission_mappings.len());
        for permission_mapping in permission_mappings {
            let permission = permission_mapping.permission_id.into();
            permissions.insert(permission);
        }

        Ok(Self {
            id: value.id,
            login: value.login,
            display_name: value.display_name,
            permissions,
            registered: value.registered,
            updated: value.updated,
            last_login: value.last_login,
        })
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
