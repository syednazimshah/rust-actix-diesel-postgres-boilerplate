use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm, TokenData};
use chrono::{Utc, Duration};
use crate::config::config::config::{JWT_SECRET, JWT_EXPIRATION};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,       // user_id
    pub role: String,      // user_role
    pub exp: usize,
}

pub fn generate_jwt_token(user_id: &i32, role: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(JWT_EXPIRATION))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        id: *user_id,
        role: role.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    ).expect("JWT token creation failed")
}

pub fn verify_jwt(token: &str) -> Option<TokenData<Claims>> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::new(Algorithm::HS256),
    ).ok()
}
