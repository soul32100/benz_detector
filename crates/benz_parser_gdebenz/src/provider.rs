use async_trait::async_trait;
use tracing::info;

use benz_core::error::ProviderError;
use benz_core::provider::FuelProvider;
use benz_models::station::Station;

use crate::client::GdeBenzClient;
use crate::mapper;

pub struct GdeBenzProvider {
    client: GdeBenzClient,
    lat: f64,
    lon: f64,
    radius_km: u32,
}

impl GdeBenzProvider {
    pub fn new(client: reqwest::Client, lat: f64, lon: f64, radius_km: u32) -> Self {
        Self {
            client: GdeBenzClient::new(client),
            lat,
            lon,
            radius_km,
        }
    }
}

#[async_trait]
impl FuelProvider for GdeBenzProvider {
    async fn fetch_stations(&self) -> Result<Vec<Station>, ProviderError> {
        info!(
            "Запрос АЗС: lat={}, lon={}, radius={}км",
            self.lat, self.lon, self.radius_km
        );

        let nearby = self
            .client
            .fetch_nearby(self.lat, self.lon, self.radius_km)
            .await
            .map_err(|e| {
                tracing::error!("Ошибка запроса к GdeBenz API: {}", e);
                ProviderError::Network
            })?;

        let stations: Vec<Station> = nearby
            .into_iter()
            .map(mapper::station_from_nearby_dto)
            .collect();

        info!("Получено АЗС: {}", stations.len());
        Ok(stations)
    }
}
