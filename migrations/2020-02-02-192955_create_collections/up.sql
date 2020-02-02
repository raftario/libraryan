CREATE TABLE collections (
    id          INTEGER   NOT NULL PRIMARY KEY,
    name        VARCHAR   NOT NULL,
    description TEXT,
    image       BLOB,
    created     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    creator     INTEGER   NOT NULL REFERENCES users (id),
    updated     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
