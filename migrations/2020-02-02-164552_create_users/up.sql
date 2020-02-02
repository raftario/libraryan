CREATE TABLE users (
    id           INTEGER     NOT NULL PRIMARY KEY,
    login        VARCHAR(16) NOT NULL UNIQUE,
    password     BINARY(32)  NOT NULL,
    display_name VARCHAR,
    registered   TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated      TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_login   TIMESTAMP
);
