CREATE TABLE user_collection_mappings (
    id            INTEGER NOT NULL PRIMARY KEY,
    user_id       INTEGER NOT NULL REFERENCES users (id),
    collection_id INTEGER NOT NULL REFERENCES collections (id)
);
