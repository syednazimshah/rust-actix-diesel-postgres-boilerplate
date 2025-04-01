pub mod config {
    pub const MODE: &str = "PRODUCTION MODE";
    pub const DATABASE_URL: &str = "postgresql://username:password@localhost:5432/database_name"; // your prod enviroment database url
    pub const PORT: u16 = 8080; // your prod enviroment port
}