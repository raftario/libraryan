CREATE TABLE book_category_mappings (
    id          INTEGER NOT NULL PRIMARY KEY,
    book_id     INTEGER NOT NULL REFERENCES books (id),
    category_id INTEGER NOT NULL REFERENCES categories (id)
);
