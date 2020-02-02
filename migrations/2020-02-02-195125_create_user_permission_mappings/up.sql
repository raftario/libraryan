CREATE TABLE user_permission_mappings (
    id            INTEGER NOT NULL PRIMARY KEY,
    user_id       INTEGER NOT NULL REFERENCES users (id),
    permission_id INTEGER NOT NULL REFERENCES permissions (id)
);
