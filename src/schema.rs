// @generated automatically by Diesel CLI.

diesel::table! {
    boards (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 260]
        path -> Varchar,
        owner_id -> Int4,
    }
}

diesel::table! {
    invitations (id) {
        id -> Int4,
        #[max_length = 50]
        code -> Varchar,
        role -> Int4,
        expire -> Timestamp,
        user_id -> Int4,
        board_id -> Int4,
    }
}

diesel::table! {
    permissions (board_id, user_id) {
        board_id -> Int4,
        user_id -> Int4,
        role -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 50]
        surname -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
    }
}

diesel::joinable!(boards -> users (owner_id));
diesel::joinable!(invitations -> boards (board_id));
diesel::joinable!(invitations -> users (user_id));
diesel::joinable!(permissions -> boards (board_id));
diesel::joinable!(permissions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    boards,
    invitations,
    permissions,
    users,
);
