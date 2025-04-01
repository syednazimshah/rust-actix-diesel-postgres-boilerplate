use diesel::prelude::*;
use super::model::*;
use crate::repository::schema::users::dsl::*;
use crate::repository::database::Database;

impl Database{
    pub fn get_users(&self) -> Vec<User> {
        users
            .load::<User>(&mut self.pool.get().unwrap())
            .expect("Error loading all users")
    }

    pub fn create_user(&self, user: _User) -> Result<User, diesel::result::Error> {
        
        let _result = diesel::insert_into(users)
            .values(&user)
            .get_result::<User>(&mut self.pool.get().unwrap());
        _result
    }

    pub fn get_user_by_id(&self, user_id: &i32) -> Result<User, diesel::result::Error> {
        let user = users
            .find(user_id)
            .get_result::<User>(&mut self.pool.get().unwrap());
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
}