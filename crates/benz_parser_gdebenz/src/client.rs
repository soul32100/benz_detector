use crate::dto::{NearbyStationDto, StationDto};

const BASE_URL: &str = "https://gdebenz.ru/api";

pub struct GdeBenzClient {
    client: reqwest::Client,
}

impl GdeBenzClient {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

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
}
