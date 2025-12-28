// Library crate for mod-aggregator
// This allows tests to import modules

pub mod commands;
pub mod database;
pub mod models;
pub mod notification;
pub mod parser;
pub mod parser_builder;
pub mod services;

// Re-export commonly used types for tests
pub use models::*;
pub use services::dependency_service::*;

