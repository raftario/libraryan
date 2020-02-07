#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Permission {
    Admin,
    RegisterUser,
    UpdateUser,
    DeleteUser,
    UploadBook,
    UpdateBook,
    DeleteBook,
    CreateAuthor,
    UpdateAuthor,
    DeleteAuthor,
    CreateCategory,
    UpdateCategory,
    DeleteCategory,
    CreateCollection,
    UpdateCollection,
    DeleteCollection,
}

impl From<i32> for Permission {
    fn from(i: i32) -> Self {
        match i {
            1 => Self::Admin,
            2 => Self::RegisterUser,
            3 => Self::UpdateUser,
            4 => Self::DeleteUser,
            5 => Self::UploadBook,
            6 => Self::UpdateBook,
            7 => Self::DeleteBook,
            8 => Self::CreateAuthor,
            9 => Self::UpdateAuthor,
            10 => Self::DeleteAuthor,
            11 => Self::CreateCategory,
            12 => Self::UpdateCategory,
            13 => Self::DeleteCategory,
            14 => Self::CreateCollection,
            15 => Self::UpdateCollection,
            16 => Self::DeleteCollection,
            _ => panic!("Invalid permission ID"),
        }
    }
}

impl Into<i32> for Permission {
    fn into(self) -> i32 {
        match self {
            Self::Admin => 1,
            Self::RegisterUser => 2,
            Self::UpdateUser => 3,
            Self::DeleteUser => 4,
            Self::UploadBook => 5,
            Self::UpdateBook => 6,
            Self::DeleteBook => 7,
            Self::CreateAuthor => 8,
            Self::UpdateAuthor => 9,
            Self::DeleteAuthor => 10,
            Self::CreateCategory => 11,
            Self::UpdateCategory => 12,
            Self::DeleteCategory => 13,
            Self::CreateCollection => 14,
            Self::UpdateCollection => 15,
            Self::DeleteCollection => 16,
        }
    }
}

impl From<db::Permission> for Permission {
    fn from(p: db::Permission) -> Self {
        Self::from(p.id)
    }
}

pub mod db {
    use crate::schema::permissions;

    #[derive(Identifiable, Queryable)]
    #[table_name = "permissions"]
    pub struct Permission {
        pub id: i32,
        pub name: String,
    }
}
