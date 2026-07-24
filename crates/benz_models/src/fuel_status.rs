use serde::{Deserialize, Serialize};

/// Состояние доступности топлива на АЗС.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum FuelStatus {
    /// Топливо есть.
    Available,

    /// Топлива мало или ограниченная доступность.
    Low,

    /// Топлива нет.
    Unavailable,

    /// Статус неизвестен.
    Unknown,
}
