/// Маппинг DTO gdebenz.ru → модели benz_models.
/// Преобразует сырые JSON-структуры в Station (без цен — gdebenz даёт только статусы).

use chrono::{DateTime, Utc};

use benz_models::brand::Brand;
use benz_models::fuel_availability::FuelAvailability;
use benz_models::fuel_status::FuelStatus;
use benz_models::fuel_type::FuelType;
use benz_models::id::canonical_station_id;
use benz_models::provider::Provider;
use benz_models::station::Station;
use benz_models::station_overall_status::StationOverallStatus;

use crate::dto::{NearbyStationDto, StationDto};

/// Маппинг статуса топлива из gdebenz. "yes" → доступно, "low" → мало, "no" → нет.
fn map_status(s: Option<&str>) -> FuelStatus {
    match s {
        Some("yes") => FuelStatus::Available,
        Some("low") => FuelStatus::Low,
        Some("no") => FuelStatus::Unavailable,
        _ => FuelStatus::Unknown,
    }
}

/// Маппинг общего статуса станции: "yes"/"low" → работает, "no" → не работает.
fn map_overall_status(s: Option<&str>) -> StationOverallStatus {
    match s {
        Some("yes") | Some("low") => StationOverallStatus::Works,
        Some("no") => StationOverallStatus::NotWorking,
        _ => StationOverallStatus::Unknown,
    }
}

/// Определение бренда по названию из gdebenz.
fn map_brand(s: Option<&str>) -> Brand {
    match s {
        Some(b) if b.contains("Лукойл") || b.eq_ignore_ascii_case("lukoil") => Brand::Lukoil,
        Some(b) if b.contains("Газпром") || b.eq_ignore_ascii_case("gazprom") => Brand::Gazprom,
        Some(b) if b.contains("Роснефть") || b.eq_ignore_ascii_case("rosneft") => Brand::Rosneft,
        Some(b) if b.contains("Татнефть") || b.eq_ignore_ascii_case("tatneft") => Brand::Tatneft,
        Some(b) if b.contains("Башнефть") || b.eq_ignore_ascii_case("bashneft") => Brand::Bashneft,
        Some(b) => Brand::Unknown(b.to_string()),
        None => Brand::Unknown(String::new()),
    }
}

/// Маппинг типа топлива из строки gdebenz ("92" → Ai92, "ДТ" → Diesel).
fn parse_fuel_type(s: &str) -> FuelType {
    match s.trim() {
        "92" => FuelType::Ai92,
        "95" => FuelType::Ai95,
        "95 Puls" | "95Plus" | "95puls" => FuelType::Ai95Puls,
        "98" => FuelType::Ai98,
        "100" => FuelType::Ai100,
        "ДТ" | "Дизель" | "diesel" => FuelType::Diesel,
        "Газ" | "газ" | "gas" | "methane" | "propane" => FuelType::Gas,
        _ => FuelType::Unknown,
    }
}

/// Парсинг поля fuels_now: строка с типами топлива через запятую ("92,95,ДТ").
fn parse_fuels_now(fuels_now: Option<&str>) -> Vec<FuelType> {
    match fuels_now {
        Some(s) if !s.is_empty() => s.split(',').map(|f| parse_fuel_type(f.trim())).collect(),
        _ => vec![],
    }
}

/// Парсинг времени last_at из формата "YYYY-MM-DD HH:MM:SS" в DateTime<Utc>.
fn parse_last_at(s: Option<&str>) -> Option<DateTime<Utc>> {
    let s = s?;
    DateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

fn now_utc() -> DateTime<Utc> {
    Utc::now()
}

/// Преобразование StationDto → Station.
pub fn station_from_dto(dto: StationDto) -> Station {
    let station_status = map_status(dto.status.as_deref());
    let fuel_types = parse_fuels_now(dto.fuels_now.as_deref());

    let fuels: Vec<FuelAvailability> = if fuel_types.is_empty() {
        vec![]
    } else {
        fuel_types
            .into_iter()
            .map(|ft| FuelAvailability {
                fuel_type: ft,
                status: station_status,
                provider: Provider::GdeBenz,
                checked_at: now_utc(),
                price: None,
            })
            .collect()
    };

    Station {
        id: canonical_station_id(&dto.osm_id),
        osm_id: dto.osm_id,
        name: dto.name.unwrap_or_default(),
        brand: map_brand(dto.brand.as_deref()),
        address: dto.addr.unwrap_or_default(),
        latitude: dto.lat,
        longitude: dto.lon,
        fuels,
        tags: vec![],
        overall_status: map_overall_status(dto.status.as_deref()),
        last_updated: None,
        reports_24h: None,
    }
}

/// Преобразование NearbyStationDto → Station.
/// checked_at устанавливается из last_at (реальное время последней отметки),
/// что позволяет UnifiedProvider корректно сравнивать свежесть данных.
pub fn station_from_nearby_dto(dto: NearbyStationDto) -> Station {
    let station_status = map_status(dto.status.as_deref());
    let fuel_types = parse_fuels_now(dto.fuels_now.as_deref());
    let last_updated = parse_last_at(dto.last_at.as_deref());
    // Используем реальное время последней отметки, а не текущее время.
    // Это критически важно для правильного merge в UnifiedProvider.
    let station_ts = last_updated.unwrap_or_else(now_utc);

    let fuels: Vec<FuelAvailability> = if fuel_types.is_empty() {
        vec![]
    } else {
        fuel_types
            .into_iter()
            .map(|ft| FuelAvailability {
                fuel_type: ft,
                status: station_status,
                provider: Provider::GdeBenz,
                checked_at: station_ts,
                price: None,
            })
            .collect()
    };

    Station {
        id: canonical_station_id(&dto.osm_id),
        osm_id: dto.osm_id,
        name: dto.name.unwrap_or_default(),
        brand: map_brand(dto.brand.as_deref()),
        address: dto.addr.unwrap_or_default(),
        latitude: dto.lat,
        longitude: dto.lon,
        fuels,
        tags: vec![],
        overall_status: map_overall_status(dto.status.as_deref()),
        last_updated,
        reports_24h: dto.confirmations,
    }
}
