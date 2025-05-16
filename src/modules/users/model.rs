
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub profile_picture_url: Option<String>,
    pub bio: Option<String>,
    pub is_active: bool,
    pub is_verified: bool,
    pub last_login: NaiveDateTime,
    pub last_login_ip: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::users)]
pub struct _User {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub profile_picture_url: Option<String>,
    pub bio: Option<String>,
    pub is_active: bool,
    pub is_verified: bool,
    pub last_login: NaiveDateTime,
    pub last_login_ip: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub profile_picture_url: Option<String>,
    pub bio: Option<String>,
    pub is_active: bool,
    pub is_verified: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct _UserLogin {
    pub email: String,
    pub password: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthToken {
    pub token: String,
    pub expires_at: NaiveDateTime,
}