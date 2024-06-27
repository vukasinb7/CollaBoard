CREATE TABLE invitations
(
    id       SERIAL PRIMARY KEY,
    code     VARCHAR(50) NOT NULL,
    role     INT         NOT NULL,
    expire   TIMESTAMP    NOT NULL,
    user_id  INT         NOT NULL,
    board_id INT         NOT NULL,
    CONSTRAINT fk_user_invitation
    FOREIGN KEY (user_id) REFERENCES users (id)
    ON DELETE SET NULL,
    CONSTRAINT fk_board_invitation
    FOREIGN KEY (board_id) REFERENCES boards (id)
    ON DELETE SET NULL
)