/// Источники данных о топливе.

use serde::{Deserialize, Serialize};

/// Какой сайт предоставил информацию.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Provider {
    /// Данные с gdebenz.ru.
    GdeBenz,
    /// Данные с benzest.ru.
    BenzEst,
}
