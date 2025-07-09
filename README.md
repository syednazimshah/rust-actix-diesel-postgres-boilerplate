# Rust Actix Boilerplate

Welcome to the Rust Actix Boilerplate! This repository serves as a solid foundation for building web applications using Rust and the Actix framework. It includes basic implementation of JWT based authentication, configurations for Diesel as the ORM, embedded database migrations, environment-based configuration management and basic user management APIs.

## Getting Started

Follow these instructions to set up and run the project on your local machine for development and testing.

## Features

This boilerplate provides the following features:

### 1. User Auth

The project features basic JWT-based user authentication. It provides two public endpoints: `/login` and `/register`. User passwords are securely hashed using the `argon2` library. Issued JWTs include claims for user ID, user role, and token expiration. An Auth Guard for authentication has been developed using JWT. It can handle user authentication and authorization efficiently, ensuring secure API access. It validates the token and make sures the user with correct role is accessing the API. `src/auth/` has auth related files.

### 1. Embedded Migrations

Diesel migrations are embedded within the project, ensuring that they run automatically when deploying the application. This eliminates the need to manually execute migration commands in staging or production environments.

### 2. Environment Configuration

The `config` directory provides environment-specific configuration files for both development and production. The application automatically loads the correct configuration based on the build environment, simplifying deployment and management.

You may choose to define environment variables directly in the `.env` file instead of using the `config` directory. However, if you plan to deploy only the compiled binary, using the `config` files is ok. Always ensure sensitive information is managed securely.

### 3. Modular MVC Architecture

The project follows an MVC (Model-View-Controller) pattern with modular design. Each module is structured as follows:

- `api.rs`: Defines API endpoints for the module.
- `db_functions.rs`: Contains database-related functions for the module.
- `model.rs`: Defines data structures corresponding to the database schema.

Additional modules can be added under the `src/modules/` directory, maintaining this structured approach.

## Project Structure

A quick overview of the project directory structure:

```sh
.
├── Cargo.toml
├── diesel.toml
├── migrations
│   ├── 00000000000000_diesel_initial_setup
│   ├── 2024-06-28-113206_create_users_table
├── README.md
└── src
    ├── auth/               # Contains logic related to JWT auth
    ├── config/             # Environment configurations
    ├── main.rs             # Application entry point
    ├── modules/            # Feature-based modules (MVC architecture)
    └── repository/         # Database-related files
```

## Prerequisites

Ensure you have the following installed on your system:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Diesel CLI](https://diesel.rs/guides/getting-started/)
- [PostgreSQL](https://www.postgresql.org/download/) (or another supported database)

## Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/syednazimshah/rust-actix-diesel-postgres-boilerplate.git
   cd rust-actix-diesel-postgres-boilerplate
   ```

2. Set up environment variables:

   Update the `.env` file in the project root with your database credentials:

   ```env
   DATABASE_URL=postgres://username:password@localhost/database_name
   ```
   
   This is required to run Diesel CLI commands in your local development environment.

3. Install Diesel CLI (PostgreSQL support):

   ```sh
   cargo install diesel_cli --no-default-features --features postgres
   ```

4. Set up the database:

   ```sh
   diesel setup
   ```

## Running the Project

To start the project in development mode, use:

   ```sh
   cargo run
   ```

## Database Migrations

### Creating a Migration

Generate a new migration using the following command:

   ```sh
   diesel migration generate migration_name
   ```

### Running Migrations

Apply all pending migrations:

   ```sh
   diesel migration run
   ```

### Reverting Migrations

Rollback the last migration:

   ```sh
   diesel migration revert
   ```

## Middleware

Middleware functionality has been developed to enhance security and request processing. The middleware will include:

- **Authentication Guard**: Ensuring secure access to protected routes.
- **Logging**: Capturing request logs for debugging and monitoring.

---

Stay tuned for updates, and feel free to contribute or report issues!