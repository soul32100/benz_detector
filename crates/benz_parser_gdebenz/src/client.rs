/// HTTP-клиент для API gdebenz.ru.

use crate::dto::{NearbyStationDto, PricesResponse, StationDto};

const BASE_URL: &str = "https://gdebenz.ru/api";

/// Клиент для запросов к API gdebenz.ru.
pub struct GdeBenzClient {
    client: reqwest::Client,
}

impl GdeBenzClient {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    /// Запросить станции по bounding box (не используется сейчас, only `fetch_nearby`).
    pub async fn fetch_stations(
        &self,
        lat1: f64,
        lon1: f64,
        lat2: f64,
        lon2: f64,
    ) -> Result<Vec<StationDto>, String> {
        let url = format!(
            "{}/stations?lat1={}&lon1={}&lat2={}&lon2={}",
            BASE_URL, lat1, lon1, lat2, lon2
        );
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Ошибка сети: {}", e))?;
        let body = resp.text().await.map_err(|e| format!("Ошибка чтения ответа: {}", e))?;
        serde_json::from_str::<Vec<StationDto>>(&body)
            .map_err(|e| format!("Ошибка парсинга JSON: {}", e))
    }

    /// Запросить станции рядом с точкой в радиусе (основной метод).
    pub async fn fetch_nearby(
        &self,
        lat: f64,
        lon: f64,
        radius_km: u32,
    ) -> Result<Vec<NearbyStationDto>, String> {
        let url = format!(
            "{}/nearby?lat={}&lon={}&radius_km={}",
            BASE_URL, lat, lon, radius_km
        );
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Ошибка сети: {}", e))?;
        let body = resp.text().await.map_err(|e| format!("Ошибка чтения ответа: {}", e))?;
        #[derive(serde::Deserialize)]
        struct NearbyResponse {
            stations: Vec<NearbyStationDto>,
        }
        let parsed: NearbyResponse = serde_json::from_str(&body)
            .map_err(|e| format!("Ошибка парсинга JSON: {}", e))?;
        Ok(parsed.stations)
    }

    /// Получить агрегированные цены в регионе (crowd-source, не per-station).
    /// Используется только для ознакомления, не для merge.
    pub async fn fetch_prices(
        &self,
        lat: f64,
        lon: f64,
    ) -> Result<PricesResponse, String> {
        let url = format!("{}/prices?lat={}&lon={}", BASE_URL, lat, lon);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Ошибка сети: {}", e))?;
        let body = resp.text().await.map_err(|e| format!("Ошибка чтения ответа: {}", e))?;
        serde_json::from_str(&body)
            .map_err(|e| format!("Ошибка парсинга JSON: {}", e))
    }
}
