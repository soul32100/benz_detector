use async_trait::async_trait;
use serde::Deserialize;

use crate::{error::LocationError, models::Location, provider::LocationProvider};

#[derive(Debug, Deserialize)]
struct IpApiResponse {
    lat: f64,
    lon: f64,
    country: Option<String>,
    city: Option<String>,
    region: Option<String>,
}

pub struct IpLocationProvider {
    client: reqwest::Client,
}

impl IpLocationProvider {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl LocationProvider for IpLocationProvider {
    async fn current_location(&self) -> Result<Location, LocationError> {
        let resp: IpApiResponse = self
            .client
            .get("http://ip-api.com/json/?fields=lat,lon,country,city,region")
            .send()
            .await
            .map_err(|e| LocationError::Network(e.to_string()))?
            .json()
            .await
            .map_err(|e| LocationError::Parse(e.to_string()))?;

        Ok(Location {
            latitude: resp.lat,
            longitude: resp.lon,
            country: resp.country,
            region: resp.region,
            city: resp.city,
        })
    }
}
