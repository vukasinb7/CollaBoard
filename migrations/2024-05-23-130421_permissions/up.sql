CREATE TABLE permissions
(
    board_id INT,
    user_id INT,
    role INT,
    PRIMARY KEY (board_id,user_id),
    CONSTRAINT fk_board_permission FOREIGN KEY(board_id)
    REFERENCES boards(id) ON DELETE SET NULL,
    CONSTRAINT fk_user_permission FOREIGN KEY(user_id)
    REFERENCES users(id) ON DELETE SET NULL
)
