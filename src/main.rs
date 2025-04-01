use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use modules::users;
use serde::Serialize;
mod config;
mod repository;
mod modules;
use crate::config::config::config::*;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}


async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("\nRunning in {}\n",MODE);
    
    let user_db = repository::database::Database::new();
    
    let result = user_db.run_migrations();
    match result {
        Ok(_res)=>{
           println!("Migrations Run Successful: {:?}\n", _res)
        }
        Err(err)=>{
            println!("Migrations Run Failed: {:?}\n", err)
        }
    }
    let app_data = web::Data::new(user_db);

    HttpServer::new(move ||
        App::new()
            .app_data(app_data.clone())
            .configure(users::api::users_config)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    )
        .bind(("127.0.0.1", PORT))?
        .run()
        .await
}
