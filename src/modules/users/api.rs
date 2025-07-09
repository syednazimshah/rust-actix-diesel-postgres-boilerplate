use actix_web::{ web, HttpResponse};
use actix_web::http::Method;
use crate::repository::database::Database;
use super::model::*;
use crate::auth::auth::auth_guard;
use crate::config::config::config::JWT_EXPIRATION;
use serde_json::json;

pub async fn create_user(db: web::Data<Database>, new_user: web::Json<CreateUser>) -> HttpResponse {
    let user = db.create_user(new_user.into_inner());
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => {
            if err.to_string().contains("duplicate key") {
                return HttpResponse::Conflict().body("User with this email already exists");
            }
            HttpResponse::InternalServerError().body(err.to_string())
        },
    }
}

pub async fn get_user_by_id(db: web::Data<Database>, id: web::Path<i32>) -> HttpResponse {
    let user = db.get_user_by_id(&id);

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
    }
}

pub async fn get_users(db: web::Data<Database>) -> HttpResponse {
    let users = db.get_users();
    HttpResponse::Ok().json(users)
}

pub async fn delete_user_by_id(db: web::Data<Database>, id: web::Path<i32>) -> HttpResponse {
    let user = db.delete_user_by_id(&id);
    match user {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body("User not found"),
    }
}

pub async fn update_user_by_id(db: web::Data<Database>, id: web::Path<i32>, updated_user: web::Json<UpdateUser>) -> HttpResponse {
    let user = db.update_user_by_id(&id, updated_user.into_inner());
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}

pub async fn login_user(db: web::Data<Database>, login_user: web::Json<UserLogin>) -> HttpResponse {
    let result = db.login_user(login_user.into_inner());
    match result {
        Ok(token) => HttpResponse::Ok().json(json!({
            "token": token,
            "expires_in_sec": JWT_EXPIRATION
        })),
        Err(_e) => HttpResponse::Unauthorized().body("Invalid email or password"),
    }
}

pub fn users_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .route("", web::route().method(Method::POST).guard(auth_guard("user")).to(create_user))
            .route("", web::route().method(Method::GET).guard(auth_guard("user")).to(get_users))
            .route("/{id}", web::route().method(Method::GET).guard(auth_guard("user")).to(get_user_by_id))
            .route("/{id}", web::route().method(Method::DELETE).guard(auth_guard("user")).to(delete_user_by_id))
            .route("/{id}", web::route().method(Method::PUT).guard(auth_guard("user")).to(update_user_by_id))
            .route("/login", web::route().method(Method::POST).to(login_user))
            .route("/register", web::route().method(Method::POST).to(create_user))
    );
}
