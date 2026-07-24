/// DTO для десериализации ответов API gdebenz.ru.

use serde::Deserialize;

/// Информация о цене одного типа топлива (crowd-source).
#[derive(Debug, Deserialize)]
pub struct PriceInfo {
    /// Средняя цена.
    pub p: f64,
    /// Число отметок.
    pub n: i32,
    /// Время последней отметки.
    pub t: Option<String>,
}

/// Агрегированные цены в регионе (не per-station).
#[derive(Debug, Deserialize)]
pub struct PricesResponse {
    /// Ключ — тип топлива ("92", "95", "ДТ").
    pub prices: std::collections::HashMap<String, PriceInfo>,
    pub source: Option<String>,
    pub disclaimer: Option<String>,
}

/// Станция из endpoint /api/stations (bbox).
#[derive(Debug, Deserialize)]
pub struct StationDto {
    pub osm_id: String,
    pub name: Option<String>,
    pub brand: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub addr: Option<String>,
    /// "yes" / "no" / "low".
    pub status: Option<String>,
    /// Какие виды топлива есть (через запятую).
    pub fuels_now: Option<String>,
    pub conflict: Option<String>,
}

/// Станция из endpoint /api/nearby (по радиусу).
#[derive(Debug, Deserialize)]
pub struct NearbyStationDto {
    pub osm_id: String,
    pub brand: Option<String>,
    pub name: Option<String>,
    pub addr: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub distance_km: Option<f64>,
    /// "yes" / "no" / "low".
    pub status: Option<String>,
    /// Текстовое описание статуса.
    pub detail: Option<String>,
    /// Какие виды топлива есть (через запятую).
    pub fuels_now: Option<String>,
    pub confirmations: Option<i32>,
    /// Время последней отметки (формат "YYYY-MM-DD HH:MM:SS").
    pub last_at: Option<String>,
    pub confidence_base: Option<f64>,
}
