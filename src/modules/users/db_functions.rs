use diesel::prelude::*;
use super::model::*;
use crate::repository::schema::users::dsl::*;
use crate::repository::database::Database;
use crate::auth::{hash::{verify_password, hash_password}, jwt::generate_jwt_token};

impl Database{
    pub fn get_users(&self) -> Vec<User> {
        let all_users = users
            .load::<User>(&mut self.pool.get().unwrap())
            .expect("Error loading all users");
        all_users.into_iter().map(|mut user| {
            user.password = "********".to_string(); // Mask the password for security
            user
        }).collect()
    }

    pub fn create_user(&self, mut user: CreateUser) -> Result<User, diesel::result::Error> {
        // Extend the temp_user with the new user data and add the values that are not in the NewUser struct
        user.is_verified = Some(false);
        user.last_login = Some(chrono::Local::now().naive_utc());
        user.created_at = Some(chrono::Local::now().naive_utc());
        user.updated_at = Some(chrono::Local::now().naive_utc());
        let pass = user.password.clone();
        user.password = hash_password(&user.password).unwrap_or_else(|_| {
            panic!("Failed to hash password for user: {}", user.email);
        });

        let _result = diesel::insert_into(users)
            .values(user)
            .get_result::<User>(&mut self.pool.get().unwrap());
        match _result {
            Ok(mut _user) => {
                _user.password = pass;
                Ok(_user)
            },
            Err(e) => Err(e),
        }
    }

    pub fn get_user_by_id(&self, user_id: &i32) -> Result<User, diesel::result::Error> {
        let user = users
            .find(user_id)
            .get_result::<User>(&mut self.pool.get().unwrap());
        match user {
            Ok(mut _user) => {
                _user.password = "********".to_string(); // Mask the password for security
                Ok(_user)
            },
            Err(e) => Err(e),
        }
    }

    fn get_user_by_email(&self, user_email: &str) -> Result<User, diesel::result::Error> {
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

    pub fn update_user_by_id(&self, user_id: &i32, mut user: UpdateUser) -> Result<User, diesel::result::Error> {
        user.updated_at = Some(chrono::Local::now().naive_utc());
        let user = diesel::update(users.find(user_id))
            .set(&user)
            .get_result::<User>(&mut self.pool.get().unwrap());
        match user {
            Ok(mut _user) => {
                _user.password = "********".to_string(); // Mask the password for security
                Ok(_user)
            },
            Err(e) => Err(e),
        }
    }

    pub fn login_user(&self, login_user: UserLogin) -> Result<String, diesel::result::Error> {
        // If email or password is empty, return an error
        if login_user.email.is_empty() || login_user.password.is_empty() {
            return Err(diesel::result::Error::NotFound);
        }
        // Fetch the user by email
        let db_user = self.get_user_by_email(&login_user.email);

        match db_user {
            Ok(user) => {
                // Verify the password
                if verify_password(&login_user.password, &user.password) {
                    // Here generate a JWT token
                    let token = generate_jwt_token(&user.id, &user.role);
                    // Update the last login time of the user
                    let updated_user = diesel::update(users.find(user.id))
                        .set(last_login.eq(chrono::Local::now().naive_utc()))
                        .get_result::<User>(&mut self.pool.get().unwrap());
                    // If the update is successful, return the token
                    match updated_user {
                        Ok(_) => Ok(token),
                        Err(e) => {
                            // If there is an error updating the last login time, log it and return an error
                            eprintln!("Error updating last login time: {}", e);
                            Err(diesel::result::Error::NotFound)
                        },
                    }
                } else {
                    // If the password verification fails, return an error
                    eprintln!("Password verification failed for user: {}", login_user.email);
                    Err(diesel::result::Error::NotFound)
                }
            },
            Err(_e) => {
                // Print the actual error for debugging purposes
                eprintln!("Error fetching user by email: {}", _e);
                Err(diesel::result::Error::NotFound)
            },
        }
    }
}