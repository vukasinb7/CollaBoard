CREATE TABLE boards
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR(50)  NOT NULL,
    path VARCHAR(260) NOT NULL,
    owner_id  INT NOT NULL,
    CONSTRAINT fk_owner_id
    FOREIGN KEY(owner_id)
    REFERENCES users(id)
    ON DELETE SET NULL
)