// @generated automatically by Diesel CLI.

diesel::table! {
    access_tokens (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        token -> Text,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 50]
        role -> Varchar,
        #[max_length = 255]
        first_name -> Nullable<Varchar>,
        #[max_length = 255]
        last_name -> Nullable<Varchar>,
        date_of_birth -> Nullable<Date>,
        #[max_length = 255]
        profile_picture_url -> Nullable<Varchar>,
        bio -> Nullable<Text>,
        is_active -> Bool,
        is_verified -> Bool,
        last_login -> Timestamp,
        #[max_length = 45]
        last_login_ip -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(access_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    access_tokens,
    users,
);
