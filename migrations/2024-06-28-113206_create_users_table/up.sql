-- Your SQL goes here
-- up.sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL,
    full_name VARCHAR(255),
    date_of_birth DATE,
    profile_picture_url VARCHAR(255),
    bio TEXT,
    is_verified BOOLEAN NOT NULL,
    last_login TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);