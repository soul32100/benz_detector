/// Описание автозаправочной станции — объединённые данные из всех источников.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::brand::Brand;
use crate::fuel_availability::FuelAvailability;
use crate::station_overall_status::StationOverallStatus;
use crate::station_tag::StationTagInfo;

/// Автозаправочная станция.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Station {
    /// Канонический UUID (v5 от osm_id).
    pub id: Uuid,
    /// OSM ID станции (сырой идентификатор из OpenStreetMap).
    pub osm_id: String,
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
    /// Доступные виды топлива с ценами и статусами.
    pub fuels: Vec<FuelAvailability>,
    /// Теги (лимит, карты, очередь и т.д.).
    pub tags: Vec<StationTagInfo>,
    /// Общий статус станции (работает/закрыта).
    pub overall_status: StationOverallStatus,
    /// Время последнего обновления данных о станции.
    pub last_updated: Option<DateTime<Utc>>,
    /// Количество отметок от водителей за последние 24 часа.
    pub reports_24h: Option<i32>,
}
