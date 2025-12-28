use crate::models::{File, FileDependency};

/// Разрешитель версий зависимостей
///
/// Обрабатывает разрешение версий зависимостей, включая поддержку
/// сосуществования нескольких версий одной зависимости.
pub struct VersionResolver;

impl VersionResolver {
    /// Создать новый разрешитель версий
    pub fn new() -> Self {
        Self
    }

    /// Получить доступные версии файла с указанным именем
    ///
    /// # Параметры
    /// * `files` - список всех файлов
    /// * `file_name` - имя файла для поиска
    ///
    /// # Возвращает
    /// Вектор версий файла
    pub fn get_available_versions(files: &[File], file_name: &str) -> Vec<String> {
        files
            .iter()
            .filter(|f| f.name == file_name)
            .map(|f| f.version.clone())
            .collect()
    }

    /// Проверить, удовлетворяет ли файл требованию зависимости
    ///
    /// # Параметры
    /// * `file` - файл для проверки
    /// * `dependency` - зависимость
    ///
    /// # Возвращает
    /// true, если файл удовлетворяет зависимости
    pub fn satisfies_dependency(file: &File, dependency: &FileDependency) -> bool {
        if file.name != dependency.target_file_name {
            return false;
        }

        match &dependency.target_file_version {
            Some(required_version) => file.version == *required_version,
            None => true, // Любая версия подходит
        }
    }

    /// Найти файлы, удовлетворяющие зависимости
    ///
    /// # Параметры
    /// * `files` - список всех файлов
    /// * `dependency` - зависимость
    ///
    /// # Возвращает
    /// Вектор файлов, удовлетворяющих зависимости
    pub fn find_satisfying_files<'a>(
        files: &'a [File],
        dependency: &FileDependency,
    ) -> Vec<&'a File> {
        files
            .iter()
            .filter(|f| Self::satisfies_dependency(f, dependency))
            .collect()
    }

    /// Проверить, разрешена ли зависимость (существует ли файл)
    ///
    /// # Параметры
    /// * `files` - список всех файлов
    /// * `dependency` - зависимость
    ///
    /// # Возвращает
    /// true, если зависимость разрешена
    pub fn is_dependency_satisfied(files: &[File], dependency: &FileDependency) -> bool {
        !Self::find_satisfying_files(files, dependency).is_empty()
    }
}

impl Default for VersionResolver {
    fn default() -> Self {
        Self::new()
    }
}
