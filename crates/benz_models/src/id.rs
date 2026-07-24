/// Канонический UUID станции — единый идентификатор на основе OSM ID.
/// Используется для дедупликации станций из разных источников.

use uuid::Uuid;

/// Фиксированный namespace для генерации UUID v5 из OSM ID.
pub const CANONICAL_NAMESPACE: Uuid = Uuid::from_u128(0x6ba7b811_9dad_11d1_80b4_00c04fd430c0);

/// Создаёт канонический UUID станции на основе её OSM ID.
/// Один и тот же OSM ID всегда даёт один и тот же UUID,
/// что позволяет объединять данные из разных источников.
pub fn canonical_station_id(osm_id: &str) -> Uuid {
    Uuid::new_v5(&CANONICAL_NAMESPACE, osm_id.as_bytes())
}
