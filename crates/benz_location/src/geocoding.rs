use crate::{error::LocationError, models::Location};

#[derive(serde::Deserialize, Debug)]
struct NominatimResult {
    lat: String,
    lon: String,
    display_name: Option<String>,
}

pub async fn search_city(client: &reqwest::Client, query: &str) -> Result<Location, LocationError> {
    let url = format!(
        "https://nominatim.openstreetmap.org/search?q={}&format=json&limit=1&accept-language=ru",
        urlencoding(query)
    );

    let resp: Vec<NominatimResult> = client
        .get(&url)
        .header("User-Agent", "benz_detector/0.1")
        .send()
        .await
        .map_err(|e| LocationError::Network(e.to_string()))?
        .json()
        .await
        .map_err(|e| LocationError::Parse(e.to_string()))?;

    let result = resp
        .into_iter()
        .next()
        .ok_or(LocationError::Internal("Город не найден".into()))?;

    let lat: f64 = result
        .lat
        .parse()
        .map_err(|e| LocationError::Parse(format!("Ошибка парсинга широты: {e}")))?;
    let lon: f64 = result
        .lon
        .parse()
        .map_err(|e| LocationError::Parse(format!("Ошибка парсинга долготы: {e}")))?;

    let city = result
        .display_name
        .map(|n| n.split(',').next().unwrap_or(&n).trim().to_string());

    Ok(Location {
        latitude: lat,
        longitude: lon,
        country: None,
        region: None,
        city,
    })
}

fn urlencoding(s: &str) -> String {
    s.replace(' ', "+")
}
