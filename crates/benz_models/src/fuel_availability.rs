/// Информация о наличии и цене одного вида топлива на АЗС.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{fuel_status::FuelStatus, fuel_type::FuelType, provider::Provider};

/// Наличие и цена конкретного топлива (92, 95, ДТ и т.д.).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FuelAvailability {
    /// Тип топлива (Ai92, Ai95, Diesel и т.д.).
    pub fuel_type: FuelType,
    /// Статус (есть / нет / мало / неизвестно).
    pub status: FuelStatus,
    /// Источник данных (GdeBenz / BenzEst).
    pub provider: Provider,
    /// Время последнего обновления этих данных (из источника).
    pub checked_at: DateTime<Utc>,
    /// Цена в рублях (None если неизвестна).
    pub price: Option<f32>,
}
