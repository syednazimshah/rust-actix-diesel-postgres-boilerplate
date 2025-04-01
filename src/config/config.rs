// config.rs

#[cfg(debug_assertions)] //Dev Mode
pub use super::dev::*;

#[cfg(not(debug_assertions))] //Production Mode
pub use super::prod::*;
