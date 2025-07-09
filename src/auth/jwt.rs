use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm, TokenData};
use chrono::{Utc, Duration};

const SECRET_KEY: &[u8] = b"your-very-secret-key-123456";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,       // user_id
    pub role: String,      // user_role
    pub exp: usize,
}

pub fn generate_jwt_token(user_id: &i32, role: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(60))
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
        &EncodingKey::from_secret(SECRET_KEY),
    ).expect("JWT token creation failed")
}

pub fn verify_jwt(token: &str) -> Option<TokenData<Claims>> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::new(Algorithm::HS256),
    ).ok()
}
