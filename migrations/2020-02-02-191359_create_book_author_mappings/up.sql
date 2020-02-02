CREATE TABLE book_author_mappings (
    id        INTEGER NOT NULL PRIMARY KEY,
    book_id   INTEGER NOT NULL REFERENCES books (id),
    author_id INTEGER NOT NULL REFERENCES authors (id)
);
