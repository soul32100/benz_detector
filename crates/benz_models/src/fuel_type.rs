/// Поддерживаемые типы топлива.

use serde::{Deserialize, Serialize};

/// Вид топлива на АЗС.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FuelType {
    /// АИ-92.
    Ai92,
    /// АИ-95.
    Ai95,
    /// АИ-95 Пульс / 95+.
    Ai95Puls,
    /// АИ-98.
    Ai98,
    /// АИ-100.
    Ai100,
    /// Дизельное топливо.
    Diesel,
    /// Газ (пропан/метан).
    Gas,
    /// Неизвестный тип топлива.
    Unknown,
}
