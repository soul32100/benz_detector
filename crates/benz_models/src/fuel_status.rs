use serde::{Deserialize, Serialize};

/// Состояние доступности топлива на АЗС.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FuelStatus {
    /// Топливо есть.
    Available,

    /// Топлива нет.
    Unavailable,

    /// Статус неизвестен.
    Unknown,
}
