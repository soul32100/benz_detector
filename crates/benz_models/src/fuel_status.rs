/// Возможные состояния доступности топлива на АЗС.

use serde::{Deserialize, Serialize};

/// Статус доступности топлива.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum FuelStatus {
    /// Топливо есть в продаже.
    Available,
    /// Топливо есть, но с ограничениями (мало, очереди и т.п.).
    Low,
    /// Топлива нет.
    Unavailable,
    /// Статус не удалось определить.
    Unknown,
}
