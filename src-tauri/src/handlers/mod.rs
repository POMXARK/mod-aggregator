//! Модуль обработчиков команд для Tauri приложения
//!
//! Этот модуль содержит все обработчики команд, разделенные по функциональным областям
//! для лучшей организации и поддерживаемости кода.

pub mod utils;
pub mod sites;
pub mod mods;
pub mod parsers;
pub mod cache;
pub mod resources;
pub mod ai;
pub mod notifications;

// Re-export commonly used handler functions for main.rs
pub use utils::*;
pub use sites::*;
pub use mods::*;
pub use parsers::*;
pub use cache::*;
pub use resources::*;
pub use ai::*;
pub use notifications::*;