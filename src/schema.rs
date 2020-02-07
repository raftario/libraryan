table! {
    authors (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        born -> Nullable<Date>,
        image -> Nullable<Binary>,
        created -> Timestamp,
        creator -> Integer,
        updated -> Timestamp,
    }
}

table! {
    book_author_mappings (id) {
        id -> Integer,
        book_id -> Integer,
        author_id -> Integer,
    }
}

table! {
    book_category_mappings (id) {
        id -> Integer,
        book_id -> Integer,
        category_id -> Integer,
    }
}

table! {
    books (id) {
        id -> Integer,
        title -> Text,
        subtitle -> Nullable<Text>,
        description -> Nullable<Text>,
        isbn -> Nullable<BigInt>,
        published -> Nullable<Date>,
        publisher -> Nullable<Text>,
        page_count -> Nullable<SmallInt>,
        image -> Nullable<Binary>,
        uploaded -> Timestamp,
        uploader -> Integer,
        updated -> Timestamp,
    }
}

table! {
    categories (id) {
        id -> Integer,
        name -> Text,
        created -> Timestamp,
        creator -> Integer,
        updated -> Timestamp,
    }
}

table! {
    collections (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        image -> Nullable<Binary>,
        created -> Timestamp,
        creator -> Integer,
        updated -> Timestamp,
    }
}

table! {
    logs (id) {
        id -> Integer,
        message -> Text,
        t -> Timestamp,
    }
}

table! {
    permissions (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    user_collection_mappings (id) {
        id -> Integer,
        user_id -> Integer,
        collection_id -> Integer,
    }
}

table! {
    user_permission_mappings (id) {
        id -> Integer,
        user_id -> Integer,
        permission_id -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        login -> Text,
        password -> Text,
        display_name -> Nullable<Text>,
        registered -> Timestamp,
        updated -> Timestamp,
        last_login -> Nullable<Timestamp>,
    }
}

joinable!(authors -> users (creator));
joinable!(book_author_mappings -> authors (author_id));
joinable!(book_author_mappings -> books (book_id));
joinable!(book_category_mappings -> books (book_id));
joinable!(book_category_mappings -> categories (category_id));
joinable!(books -> users (uploader));
joinable!(categories -> users (creator));
joinable!(collections -> users (creator));
joinable!(user_collection_mappings -> collections (collection_id));
joinable!(user_collection_mappings -> users (user_id));
joinable!(user_permission_mappings -> permissions (permission_id));
joinable!(user_permission_mappings -> users (user_id));

allow_tables_to_appear_in_same_query!(
    authors,
    book_author_mappings,
    book_category_mappings,
    books,
    categories,
    collections,
    logs,
    permissions,
    user_collection_mappings,
    user_permission_mappings,
    users,
);
