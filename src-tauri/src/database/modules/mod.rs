//! Модули базы данных
//!
//! Этот модуль содержит разделение функциональности базы данных
//! по логическим областям для лучшей организации и поддержки.

pub mod base;
pub mod sites;
pub mod mods;
pub mod notifications;
pub mod pages;
pub mod files;
pub mod dependencies;
pub mod collections;
pub mod session;

// Re-export commonly used types
pub use base::*;
pub use sites::*;
pub use mods::*;
pub use notifications::*;
pub use pages::*;
pub use files::*;
pub use dependencies::*;
pub use collections::*;
pub use session::*;