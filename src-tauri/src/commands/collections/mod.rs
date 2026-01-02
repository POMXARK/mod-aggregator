//! Команды управления коллекциями
//!
//! Этот модуль объединяет все подмодули для работы с коллекциями,
//! предоставляя унифицированный интерфейс для операций с коллекциями.

pub mod models;
pub mod collection_management;
pub mod collection_files;
pub mod collection_logic;
pub mod collection_operations;

// Re-export основных структур и функций для удобства использования
pub use models::*;
pub use collection_management::*;
pub use collection_files::*;
pub use collection_logic::*;
pub use collection_operations::*;

// Re-export типов из моделей для обратной совместимости
pub use models::types::*;