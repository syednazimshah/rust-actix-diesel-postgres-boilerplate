use actix_web::{ delete, get, post, put, web, HttpResponse, HttpRequest };
use crate::repository::database::Database;
use super::model::*;

#[post("")]
pub async fn create_user(db: web::Data<Database>, new_user: web::Json<NewUser>) -> HttpResponse {
    let user = db.create_user(new_user.into_inner());
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/{id}")]
pub async fn get_user_by_id(db: web::Data<Database>, id: web::Path<i32>) -> HttpResponse {
    let user = db.get_user_by_id(&id);

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
    }
}

#[get("")]
pub async fn get_users(db: web::Data<Database>) -> HttpResponse {
    let users = db.get_users();
    HttpResponse::Ok().json(users)
}

#[delete("/{id}")]
pub async fn delete_user_by_id(db: web::Data<Database>, id: web::Path<i32>) -> HttpResponse {
    let user = db.delete_user_by_id(&id);
    match user {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body("User not found"),
    }
}

#[put("/{id}")]
pub async fn update_user_by_id(db: web::Data<Database>, id: web::Path<i32>, updated_user: web::Json<User>) -> HttpResponse {
    let user = db.update_user_by_id(&id, updated_user.into_inner());
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}

#[post("/login")]
pub async fn login_user(
    req: HttpRequest,
    db: web::Data<Database>,
    login_user: web::Json<_UserLogin>,
) -> HttpResponse {
    match req.connection_info().realip_remote_addr() {
        Some(ip) => {
            let result = db.login_user(login_user.into_inner(), ip.to_string());
            match result {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(e) => HttpResponse::NotFound().body(e.to_string()),
            }
        }
        None => {
            HttpResponse::BadRequest().body("Unable to determine client IP address")
        }
    }
}


pub fn users_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .service(create_user)
            .service(get_user_by_id)
            .service(get_users)
            .service(delete_user_by_id)
            .service(update_user_by_id)
            .service(login_user)
    );
}
