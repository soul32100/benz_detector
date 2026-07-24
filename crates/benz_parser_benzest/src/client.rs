/// HTTP-клиент для API benzest.ru.

const BASE_URL: &str = "https://benzest.ru/api";

/// Клиент для запросов к API benzest.ru.
pub struct BenzEstClient {
    client: reqwest::Client,
}

impl BenzEstClient {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    /// Запросить станции внутри bounding box.
    ///
    /// API ограничивает размер bbox — не более 4 квадратных градусов.
    /// Возвращает список станций с ценами, тегами и комментариями.
    pub async fn fetch_stations(
        &self,
        lat1: f64,
        lon1: f64,
        lat2: f64,
        lon2: f64,
    ) -> Result<Vec<crate::dto::BenzEstStationDto>, String> {
        let url = format!(
            "{}/stations?bbox={},{},{},{}",
            BASE_URL, lat1, lon1, lat2, lon2
        );
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Ошибка сети: {}", e))?;
        let body = resp
            .text()
            .await
            .map_err(|e| format!("Ошибка чтения ответа: {}", e))?;
        serde_json::from_str(&body)
            .map_err(|e| format!("Ошибка парсинга JSON: {}", e))
    }
}
