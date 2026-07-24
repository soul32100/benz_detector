use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StationDto {
    pub osm_id: String,
    pub name: Option<String>,
    pub brand: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub addr: Option<String>,
    pub status: Option<String>,
    pub fuels_now: Option<String>,
    pub conflict: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NearbyStationDto {
    pub osm_id: String,
    pub brand: Option<String>,
    pub name: Option<String>,
    pub addr: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub distance_km: Option<f64>,
    pub status: Option<String>,
    pub detail: Option<String>,
    pub fuels_now: Option<String>,
    pub confirmations: Option<i32>,
    pub last_at: Option<String>,
    pub confidence_base: Option<f64>,
}
