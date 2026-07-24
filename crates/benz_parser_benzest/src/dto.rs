/// DTO для десериализации ответов API benzest.ru.

use serde::Deserialize;

/// Станция из API benzest.
#[derive(Debug, Deserialize)]
pub struct BenzEstStationDto {
    pub id: String,
    pub name: Option<String>,
    pub address: Option<String>,
    pub lat: Option<f64>,
    #[serde(rename = "lng")]
    pub lon: Option<f64>,
    pub fuels: Vec<BenzEstFuelDto>,
    /// Статус станции: "WORKS" / "NOT_WORKING".
    pub status: Option<String>,
    pub tags: Option<Vec<BenzEstTagDto>>,
    pub comments: Option<Vec<BenzEstCommentDto>>,
    /// ISO 8601 timestamp последнего обновления.
    pub last_updated: Option<String>,
    /// Число отметок за 24ч.
    pub reports_24h: Option<i32>,
    /// ISO 8601 timestamp последней отметки.
    pub last_report_at: Option<String>,
}

/// Вид топлива с ценой и статусом.
#[derive(Debug, Deserialize)]
pub struct BenzEstFuelDto {
    #[serde(rename = "type")]
    pub fuel_type: String,
    /// "AVAILABLE" / "OUT_OF_STOCK" / "UNKNOWN".
    pub status: String,
    pub confidence: Option<String>,
    pub reports_in_window: Option<i32>,
    pub avail_prob: Option<i32>,
    pub price: Option<f64>,
    /// ISO 8601 timestamp обновления цены.
    pub price_updated_at: Option<String>,
}

/// Тег АЗС (лимит, карты, закрыто, etc).
#[derive(Debug, Deserialize)]
pub struct BenzEstTagDto {
    pub tag: String,
    pub value: Option<String>,
    /// ISO 8601 timestamp обновления тега.
    pub updated_at: Option<String>,
}

/// Комментарий от водителя.
#[derive(Debug, Deserialize)]
pub struct BenzEstCommentDto {
    pub id: String,
    /// Текст комментария.
    pub text: Option<String>,
    /// ISO 8601 timestamp.
    pub created_at: Option<String>,
}
