use crate::{brand::Brand, fuel_availability::FuelAvailability};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Описание автозаправочной станции.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Station {
    /// Уникальный идентификатор станции.
    pub id: Uuid,

    /// Название АЗС.
    pub name: String,

    /// Бренд сети.
    pub brand: Brand,

    /// Полный адрес.
    pub address: String,

    /// Географическая широта.
    pub latitude: Option<f64>,

    /// Географическая долгота.
    pub longitude: Option<f64>,

    /// Доступные виды топлива.
    pub fuels: Vec<FuelAvailability>,
}
