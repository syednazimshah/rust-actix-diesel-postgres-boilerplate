use actix_web::guard::{self, GuardContext};
use super::jwt::verify_jwt;

// Auth guard will check for token in the header, extract the user_id, role and expiration from the token.
// It will then check if the token is valid and not expired. It will also check if the user exists in the database and has the required role.
// If the conditions are met, it will allow the request to proceed, otherwise it will return an error response of 401 Unauthorized.
pub fn auth_guard(role: &str) -> impl guard::Guard + use<'_> {
    guard::fn_guard(move |ctx: &GuardContext| {
        // Log the request method and path for debugging purposes along with time and date
        let method = &ctx.head().method;
        let path = &ctx.head().uri;
        let now = chrono::Utc::now();
        println!("Method: {} - Path: {} - accessed at {}", method, path, now);
        if let Some(auth_header) = ctx.head().headers().get("Authorization") {
            if let Ok(token) = auth_header.to_str() {
                if let Some(token_data) = verify_jwt(token.trim_start_matches("Bearer ")) {
                    if token_data.claims.role == role
                        && token_data.claims.exp > chrono::Utc::now().timestamp() as usize
                    {
                        return true;
                    }
                }
            }
        }
        // If the token is not valid or expired, return false to block the request. This will trigger the default service to return a 401 Unauthorized response.
        false
    })
}
