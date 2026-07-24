use thiserror::Error;

#[derive(Debug, Error)]
pub enum LocationError {
    #[error("Не удалось определить местоположение")]
    Unavailable,

    #[error("Нет разрешения на получение геолокации")]
    PermissionDenied,

    #[error("Ошибка сети: {0}")]
    Network(String),

    #[error("Ошибка парсинга ответа: {0}")]
    Parse(String),

    #[error("Внутренняя ошибка: {0}")]
    Internal(String),
}
