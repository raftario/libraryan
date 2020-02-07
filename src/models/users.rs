use crate::{
    error::Error,
    globals::{POOL, USER_CACHE},
    models::permissions::Permission,
    schema,
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use std::{
    collections::HashSet,
    convert::{TryFrom, TryInto},
};

#[derive(Clone, Debug)]
pub struct User {
    pub id: i32,
    pub login: String,
    password: String,
    pub display_name: String,
    pub permissions: HashSet<Permission>,
    pub registered: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub last_login: Option<NaiveDateTime>,
}

impl User {
    pub fn by_id(id: i32) -> Result<User, Error> {
        let mut cache = USER_CACHE.lock().expect("Can't lock mutex");
        let user = cache.get(&id);
        if let Some(u) = user {
            return Ok((*u).clone());
        }

        let connection = POOL.get().unwrap();

        let user: User = schema::users::dsl::users
            .find(id)
            .first::<db::User>(&connection)?
            .try_into()?;
        cache.put(user.id, user.clone());
        Ok(user)
    }

    pub fn verify_passwd(&self, passwd: &[u8]) -> Result<bool, Error> {
        Ok(argon2::verify_encoded(&self.password, passwd)?)
    }
}

impl TryFrom<db::User> for User {
    type Error = Error;

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
            login: value.login.clone(),
            password: value.password,
            display_name: value.display_name.unwrap_or(value.login),
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
        pub password: String,
        pub display_name: Option<String>,
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
