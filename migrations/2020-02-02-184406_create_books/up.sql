CREATE TABLE books (
    id          INTEGER   NOT NULL PRIMARY KEY,
    title       VARCHAR   NOT NULL,
    subtitle    VARCHAR,
    description TEXT,
    isbn        BIGINT,
    published   DATE,
    publisher   VARCHAR,
    page_count  SMALLINT,
    image       BLOB,
    uploaded    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    uploader    INTEGER   NOT NULL REFERENCES users (id),
    updated     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
