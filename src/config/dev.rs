pub mod config {
    pub const MODE: &str = "DEVELOPMENT MODE";
    pub const DATABASE_URL: &str = "postgresql://username:password@localhost:5432/database_name"; // your dev enviroment database url
    pub const PORT: u16 = 8888; // your dev enviroment port
    pub const HASH_SALT: &str = "c29tZV9yYW5kb21fc2FsdA"; // your dev enviroment hash salt (Base64 encoded)
    pub const JWT_SECRET: &[u8] = b"your-very-secret-key-123456"; // your dev enviroment JWT secret key
    pub const JWT_EXPIRATION: i64 = 3600; // JWT expiration time in seconds
}