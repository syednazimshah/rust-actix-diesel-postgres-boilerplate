// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 50]
        role -> Varchar,
        #[max_length = 255]
        full_name -> Nullable<Varchar>,
        date_of_birth -> Nullable<Date>,
        #[max_length = 255]
        profile_picture_url -> Nullable<Varchar>,
        bio -> Nullable<Text>,
        is_verified -> Bool,
        last_login -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
