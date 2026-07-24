/// Провайдер benzest.ru — реализует FuelProvider через API benzest.

use async_trait::async_trait;
use tracing::info;

use benz_core::error::ProviderError;
use benz_core::provider::FuelProvider;
use benz_models::station::Station;

use crate::client::BenzEstClient;
use crate::mapper;

/// Провайдер, загружающий данные с benzest.ru.
///
/// API использует bounding box, не радиус.
/// Bounding box вычисляется из центральной точки и радиуса.
pub struct BenzEstProvider {
    client: BenzEstClient,
    lat: f64,
    lon: f64,
    radius_km: u32,
}

impl BenzEstProvider {
    pub fn new(client: reqwest::Client, lat: f64, lon: f64, radius_km: u32) -> Self {
        Self {
            client: BenzEstClient::new(client),
            lat,
            lon,
            radius_km,
        }
    }

    /// Пересчёт радиуса в bounding box.
    fn bbox(&self) -> (f64, f64, f64, f64) {
        // 1 градус широты ≈ 111.32 км
        let km_per_lat = 111.32;
        let lat_rad = self.lat.to_radians();
        // 1 градус долготы зависит от широты
        let km_per_lon = 111.32 * lat_rad.cos();

        let dlat = self.radius_km as f64 / km_per_lat;
        let dlon = self.radius_km as f64 / km_per_lon;

        (self.lat - dlat, self.lon - dlon, self.lat + dlat, self.lon + dlon)
    }
}

#[async_trait]
impl FuelProvider for BenzEstProvider {
    async fn fetch_stations(&self) -> Result<Vec<Station>, ProviderError> {
        info!(
            "Запрос АЗС (benzest): lat={}, lon={}, radius={}км",
            self.lat, self.lon, self.radius_km
        );

        let (lat1, lon1, lat2, lon2) = self.bbox();
        let stations_dto = self
            .client
            .fetch_stations(lat1, lon1, lat2, lon2)
            .await
            .map_err(|e| {
                tracing::error!("Ошибка запроса к BenzEst API: {}", e);
                ProviderError::Network
            })?;

        let stations: Vec<Station> = stations_dto
            .into_iter()
            .map(mapper::station_from_dto)
            .collect();

        info!("Получено АЗС (benzest): {}", stations.len());
        Ok(stations)
    }
}
