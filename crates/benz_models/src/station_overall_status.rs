/// Общий статус АЗС (работает / не работает / неизвестно).

use serde::{Deserialize, Serialize};

/// Состояние станции в целом (независимо от наличия конкретных видов топлива).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StationOverallStatus {
    /// Станция работает.
    Works,
    /// Станция не работает (закрыта, нет персонала и т.д.).
    NotWorking,
    /// Статус не определён.
    Unknown,
}
