use chrono::{DateTime, Utc};
use uuid::Uuid;

use benz_models::brand::Brand;
use benz_models::fuel_availability::FuelAvailability;
use benz_models::fuel_status::FuelStatus;
use benz_models::fuel_type::FuelType;
use benz_models::provider::Provider;
use benz_models::station::Station;

use crate::dto::{NearbyStationDto, StationDto};

const NAMESPACE_GDEBENZ: Uuid = Uuid::from_u128(0x6ba7b811_9dad_11d1_80b4_00c04fd430c8);

fn osm_id_to_uuid(osm_id: &str) -> Uuid {
    Uuid::new_v5(&NAMESPACE_GDEBENZ, osm_id.as_bytes())
}

fn map_status(s: Option<&str>) -> FuelStatus {
    match s {
        Some("yes") => FuelStatus::Available,
        Some("low") => FuelStatus::Low,
        Some("no") => FuelStatus::Unavailable,
        _ => FuelStatus::Unknown,
    }
}

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

fn parse_fuels_now(fuels_now: Option<&str>) -> Vec<FuelType> {
    match fuels_now {
        Some(s) if !s.is_empty() => s.split(',').map(|f| parse_fuel_type(f.trim())).collect(),
        _ => vec![],
    }
}

fn now_utc() -> DateTime<Utc> {
    Utc::now()
}

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
        id: osm_id_to_uuid(&dto.osm_id),
        name: dto.name.unwrap_or_default(),
        brand: map_brand(dto.brand.as_deref()),
        address: dto.addr.unwrap_or_default(),
        latitude: dto.lat,
        longitude: dto.lon,
        fuels,
    }
}

pub fn station_from_nearby_dto(dto: NearbyStationDto) -> Station {
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
        id: osm_id_to_uuid(&dto.osm_id),
        name: dto.name.unwrap_or_default(),
        brand: map_brand(dto.brand.as_deref()),
        address: dto.addr.unwrap_or_default(),
        latitude: dto.lat,
        longitude: dto.lon,
        fuels,
    }
}
