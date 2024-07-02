-- Your SQL goes here
CREATE TABLE users
(
    id       SERIAL PRIMARY KEY,
    name     VARCHAR(50)         NOT NULL,
    surname  VARCHAR(50)         NOT NULL,
    email    VARCHAR(100) UNIQUE NOT NULL,
    password VARCHAR(255)        NOT NULL
)