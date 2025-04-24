extern crate diesel_migrations;
use std::error::Error;
use diesel::migration::MigrationVersion;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use diesel_migrations::{ embed_migrations, EmbeddedMigrations, MigrationHarness };
use crate::config::config::config::*;

type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations"); // embedding the migration into the binary. These will run automatically when the binary in executed

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
        println!("✅ Database pool created successfully");
        Database { pool }
    }

    pub fn run_migrations(&self) -> Result<Vec<MigrationVersion<'static>>, Box<dyn Error + Send + Sync + 'static>> {
        let mut conn = PgConnection::establish(DATABASE_URL)
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

    
        let result = conn
            .run_pending_migrations(MIGRATIONS)
            .map_err(|e| format!("Failed to run migrations: {}", e))?;
    
        // Convert references to owned/static versions
        Ok(result.into_iter().map(|m| m.as_owned()).collect())
    }

}
