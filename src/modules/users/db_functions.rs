use diesel::prelude::*;
use super::model::*;
use crate::repository::schema::users::dsl::*;
use crate::repository::database::Database;
use crate::helpers::{hash::{hash_password, verify_password}, token::generate_auth_token};

impl Database{
    pub fn get_users(&self) -> Vec<User> {
        users
            .load::<User>(&mut self.pool.get().unwrap())
            .expect("Error loading all users")
    }

    pub fn create_user(&self, user: NewUser) -> Result<User, diesel::result::Error> {
        // Extend the temp_user with the new user data and add the values that are not in the NewUser struct
        let new_user: _User = _User {
            username: user.username.clone(),
            email: user.email.clone(),
            password_hash: hash_password(&user.password).unwrap(),
            role: user.role.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            date_of_birth: user.date_of_birth,
            profile_picture_url: user.profile_picture_url.clone(),
            bio: user.bio.clone(),
            is_active: true,
            is_verified: false,
            last_login: chrono::Local::now().naive_utc(),
            last_login_ip: None,
            created_at: chrono::Local::now().naive_utc(),
            updated_at: chrono::Local::now().naive_utc(),
        };

        let _result = diesel::insert_into(users)
            .values(&new_user)
            .get_result::<User>(&mut self.pool.get().unwrap());
        _result
    }

    pub fn get_user_by_id(&self, user_id: &i32) -> Result<User, diesel::result::Error> {
        let user = users
            .find(user_id)
            .get_result::<User>(&mut self.pool.get().unwrap());
        user
    }

    pub fn get_user_by_email(&self, user_email: String) -> Result<User, diesel::result::Error> {
        let user = users
            .filter(email.eq(user_email))
            .first::<User>(&mut self.pool.get().unwrap());
        user
    }

    pub fn delete_user_by_id(&self, user_id: &i32) -> Option<usize> {
        let count = diesel::delete(users.find(user_id))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting user by id");
        Some(count)
    }

    pub fn update_user_by_id(&self, user_id: &i32, user: User) -> Result<User, diesel::result::Error> {
        let user = diesel::update(users.find(user_id))
            .set(&user)
            .get_result::<User>(&mut self.pool.get().unwrap());
        user
    }

    pub fn login_user(&self, login_user: _UserLogin, ip: String) -> Result<User, diesel::result::Error> {
        let user_email = login_user.email.clone();
        let user_password = login_user.password.clone();
        let user = self.get_user_by_email(user_email);
        match user {
            Ok(mut user) => {
                if verify_password(&user_password, &user.password_hash).unwrap() {
                    user.last_login_ip = Some(ip);
                    user.last_login = chrono::Local::now().naive_utc();
                    let updated_user  = self.update_user_by_id(&user.id.clone(), user);
                    updated_user
                } else {
                    Err(diesel::result::Error::NotFound)
                }
            }
            Err(e) => Err(e),
        }
    }
}