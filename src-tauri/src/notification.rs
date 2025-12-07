use crate::models::{ModUpdate, Notification};
use crate::database::Database;

/// Сервис для работы с уведомлениями
/// 
/// Предоставляет методы для создания и отправки уведомлений об обновлениях модов
pub struct NotificationService {
    app_handle: tauri::AppHandle,
}

impl NotificationService {
    /// Создать новый экземпляр сервиса уведомлений
    /// 
    /// # Параметры
    /// * `app_handle` - handle приложения Tauri для отправки системных уведомлений
    /// 
    /// # Возвращает
    /// Новый экземпляр NotificationService
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        NotificationService { app_handle }
    }

    /// Создать и отправить уведомление об обновлении мода
    /// 
    /// Создает запись в базе данных и отправляет системное уведомление.
    /// 
    /// # Параметры
    /// * `update` - информация об обновлении мода
    /// 
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn notify_update(&self, update: &ModUpdate) -> Result<(), Box<dyn std::error::Error>> {
        let db = Database::new().await?;
        
        let title = format!("Обновление мода");
        let message = if let (Some(old_v), Some(new_v)) = (&update.old_version, &update.new_version) {
            format!("Версия изменена: {} → {}", old_v, new_v)
        } else {
            "Мод обновлен".to_string()
        };

        let notification = Notification {
            id: 0,
            mod_id: update.mod_id,
            site_id: update.site_id,
            title,
            message,
            read: false,
            created_at: chrono::Utc::now(),
        };

        db.add_notification(&notification).await?;

        // Show system notification
        #[cfg(not(target_os = "android"))]
        {
            use tauri_plugin_notification::NotificationExt;
            self.app_handle
                .notification()
                .builder()
                .title(&notification.title)
                .body(&notification.message)
                .show()?;
        }

        Ok(())
    }
}

