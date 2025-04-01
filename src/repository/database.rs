extern crate diesel_migrations;
use std::error::Error;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use diesel_migrations::{ embed_migrations, EmbeddedMigrations, MigrationHarness };
use crate::config::config::config::*;

type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations"); //embedding the migration into the binary. These will run automatically when the binary in executed

pub struct Database {
    pub pool: DBPool,
}

impl Database {

    pub fn new() -> Self {
        dotenv().ok();
        let database_url = DATABASE_URL;
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }
    pub fn run_migrations(&self) -> Result<(), Box<dyn Error + Send + Sync + 'static>> { // function to run embedded migrations
        let mut conn = PgConnection::establish(DATABASE_URL).unwrap();
        println!("Running Migrations\n");
        let _result = conn.run_pending_migrations(MIGRATIONS).unwrap();
        println!("Migrations Result: {:?}\n", _result);
        Ok(())
    }

}
