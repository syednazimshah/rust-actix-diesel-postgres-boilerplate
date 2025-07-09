
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::users)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub role: String,
    pub full_name: Option<String>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub profile_picture_url: Option<String>,
    pub bio: Option<String>,
    pub is_verified: bool,
    pub last_login: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::users)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
    pub role: String,
    pub full_name: Option<String>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub profile_picture_url: Option<String>,
    pub bio: Option<String>,
    pub is_verified: Option<bool>,
    pub last_login: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}