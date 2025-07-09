use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use modules::users;
use serde::Serialize;

mod config;
mod repository;
mod modules;
mod auth;

use crate::config::config::config::*;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Actix is up and running".to_string(),
    };
    HttpResponse::Ok().json(response)
}

async fn unauthorized() -> Result<HttpResponse> {
    let response = Response {
        message: "Unauthorized".to_string(),
    };
    Ok(HttpResponse::Unauthorized().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("\nRunning in {}\n",MODE);
    
    let user_db = repository::database::Database::new();

    match user_db.run_migrations() {
        Ok(res) if res.is_empty() => println!("No new migrations were run.\n"),
        Ok(res) => println!("✅ Ran migrations: {:?}\n", res),
        Err(err) => panic!("❌ Migration failed: {}\n", err),
    }
    let app_data = web::Data::new(user_db);

    print!("Starting Actix server on port {}...\n", PORT);

    HttpServer::new(move ||
        App::new()
            .app_data(app_data.clone())
            .configure(users::api::users_config)
            .service(healthcheck)
            .default_service(web::route().to(unauthorized))
            .wrap(actix_web::middleware::Logger::default())
    )
        .bind(("127.0.0.1", PORT))?
        .run()
        .await
}
