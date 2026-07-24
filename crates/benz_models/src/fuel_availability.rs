use crate::{fuel_status::FuelStatus, fuel_type::FuelType, provider::Provider};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Информация о наличии конкретного топлива.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FuelAvailability {
    /// Тип топлива.
    pub fuel_type: FuelType,

    /// Состояние топлива.
    pub status: FuelStatus,

    /// Источник информации.
    pub provider: Provider,

    /// Время проверки.
    pub checked_at: DateTime<Utc>,

    /// Цена топлива.
    pub price: Option<f32>,
}
