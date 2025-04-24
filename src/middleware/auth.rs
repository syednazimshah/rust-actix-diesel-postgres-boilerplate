// src/middleware/auth.rs

use actix_web::{dev::{ServiceRequest, ServiceResponse, Transform, forward_ready}, Error, HttpMessage};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use diesel::prelude::*;
use chrono::Utc;
use std::rc::Rc;
use std::task::{Context, Poll};
use std::future::ready;

use crate::{DbPool, models::User, schema::auth_tokens::dsl::*};

#[derive(Clone)]
pub struct AuthMiddlewareFactory {
    pub db_pool: DbPool,
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddlewareFactory
where
    S: actix_service::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware {
            service: Rc::new(service),
            db_pool: self.db_pool.clone(),
        })
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
    db_pool: DbPool,
}

impl<S, B> actix_service::Service<ServiceRequest> for AuthMiddleware<S>
where
    S: actix_service::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let db_pool = self.db_pool.clone();
        let service = Rc::clone(&self.service);

        let auth_header = req.headers().get("Authorization").cloned();

        Box::pin(async move {
            if let Some(header_value) = auth_header {
                if let Ok(auth_str) = header_value.to_str() {
                    let token_str = auth_str.trim_start_matches("Bearer ").to_string();

                    let conn = &mut db_pool.get().unwrap();

                    // Check token in database
                    use crate::schema::auth_tokens::dsl::*;
                    use crate::schema::users::dsl as users_dsl;

                    let result = auth_tokens
                        .filter(token.eq(&token_str))
                        .filter(expires_at.gt(Utc::now().naive_utc()))
                        .first::<crate::models::AuthToken>(conn)
                        .optional()?;

                    if let Some(auth_token) = result {
                        let user = users_dsl::users
                            .find(auth_token.user_id)
                            .first::<User>(conn)?;

                        // Attach user to request
                        req.extensions_mut().insert(user);
                    }
                }
            }

            service.call(req).await
        })
    }
}
