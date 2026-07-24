/// Маппинг DTO benzest.ru → модели benz_models.
/// Преобразует сырые JSON-структуры в Station с ценами, тегами и статусами.

use chrono::{DateTime, Utc};
use regex::Regex;

use benz_models::brand::Brand;
use benz_models::fuel_availability::FuelAvailability;
use benz_models::fuel_status::FuelStatus;
use benz_models::fuel_type::FuelType;
use benz_models::id::canonical_station_id;
use benz_models::provider::Provider;
use benz_models::station::Station;
use benz_models::station_overall_status::StationOverallStatus;
use benz_models::station_tag::{StationTag, StationTagInfo, TagSource};

use crate::dto::{BenzEstCommentDto, BenzEstFuelDto, BenzEstStationDto};

/// Маппинг статуса топлива из API benzest.
fn map_fuel_status(s: &str) -> FuelStatus {
    match s {
        "AVAILABLE" => FuelStatus::Available,
        "LIMITED" => FuelStatus::Low,
        "OUT_OF_STOCK" => FuelStatus::Unavailable,
        _ => FuelStatus::Unknown,
    }
}

/// Определение бренда по названию станции.
fn map_brand(name: &str) -> Brand {
    match name {
        n if n.contains("Лукойл") || n.eq_ignore_ascii_case("lukoil") => Brand::Lukoil,
        n if n.contains("Газпром") || n.eq_ignore_ascii_case("gazprom") => Brand::Gazprom,
        n if n.contains("Роснефть") || n.eq_ignore_ascii_case("rosneft") => Brand::Rosneft,
        n if n.contains("Татнефть") || n.eq_ignore_ascii_case("tatneft") => Brand::Tatneft,
        n if n.contains("Башнефть") || n.eq_ignore_ascii_case("bashneft") => Brand::Bashneft,
        _ => Brand::Unknown(name.to_string()),
    }
}

/// Маппинг типа топлива из строки API benzest ("AI-92" → Ai92).
fn map_fuel_type(s: &str) -> FuelType {
    match s {
        "AI-92" => FuelType::Ai92,
        "AI-95" => FuelType::Ai95,
        "AI-95+" => FuelType::Ai95Puls,
        "AI-98" => FuelType::Ai98,
        "AI-100" => FuelType::Ai100,
        "Diesel" => FuelType::Diesel,
        _ => FuelType::Unknown,
    }
}

/// Маппинг общего статуса станции.
fn map_overall_status(s: Option<&str>) -> StationOverallStatus {
    match s {
        Some("WORKS") => StationOverallStatus::Works,
        Some("NOT_WORKING") => StationOverallStatus::NotWorking,
        _ => StationOverallStatus::Unknown,
    }
}

/// Маппинг одного тега из DTO в StationTag.
/// "limit" → FuelLimit(значение), "cards_only" → CardsOnly и т.д.
fn map_tag(tag: &str, value: Option<&str>) -> Option<StationTag> {
    match tag {
        "cards_only" => Some(StationTag::CardsOnly),
        "cash_and_cards" => Some(StationTag::CashAndCards),
        "limit" => {
            let n: u32 = value.and_then(|v| v.parse().ok()).unwrap_or(0);
            if n > 0 {
                Some(StationTag::FuelLimit(n))
            } else {
                None
            }
        }
        "no_limit" => Some(StationTag::NoLimit),
        "closed" => Some(StationTag::Closed),
        "canister_ok" => Some(StationTag::CanisterOk),
        "tank_only" => Some(StationTag::TankOnly),
        "even_odd" => Some(StationTag::EvenOdd),
        _ => None,
    }
}

/// Парсинг времени тега из ISO 8601.
fn parse_tag_time(s: Option<&str>) -> Option<DateTime<Utc>> {
    let s = s?;
    DateTime::parse_from_rfc3339(s).ok().map(|dt| dt.with_timezone(&Utc))
}

/// Парсинг времени комментария из ISO 8601.
fn parse_comment_time(s: Option<&str>) -> Option<DateTime<Utc>> {
    let s = s?;
    DateTime::parse_from_rfc3339(s).ok().map(|dt| dt.with_timezone(&Utc))
}

/// Сканирование комментариев водителей для извлечения тегов:
///   - "очередь 20+" → Queue20Plus
///   - "очередь 50+" → Queue50Plus
///   - "большая очередь" → BigQueue
///   - "лимит N" → FuelLimit(N)
fn scan_comments_for_tags(comments: &[BenzEstCommentDto]) -> Vec<StationTagInfo> {
    let now = Utc::now();
    let mut results = Vec::new();

    let queue_large = Regex::new(r"(?i)больш[ая]+\s*очеред").unwrap();
    let queue_20 = Regex::new(r"(?i)очеред[ьи]*\s*20").unwrap();
    let queue_50 = Regex::new(r"(?i)очеред[ьи]*\s*50").unwrap();
    let limit_re = Regex::new(r"(?i)лимит\s*(\d+)").unwrap();

    for comment in comments {
        let text = match &comment.text {
            Some(t) => t,
            None => continue,
        };
        let created = parse_comment_time(comment.created_at.as_deref());

        if queue_20.is_match(text) || (queue_large.is_match(text) && text.contains("20")) {
            results.push(StationTagInfo {
                tag: StationTag::Queue20Plus,
                source: TagSource::Comment,
                updated_at: created.unwrap_or(now),
            });
        }
        if queue_50.is_match(text) {
            results.push(StationTagInfo {
                tag: StationTag::Queue50Plus,
                source: TagSource::Comment,
                updated_at: created.unwrap_or(now),
            });
        }
        if queue_large.is_match(text) && !queue_20.is_match(text) && !queue_50.is_match(text) {
            results.push(StationTagInfo {
                tag: StationTag::BigQueue,
                source: TagSource::Comment,
                updated_at: created.unwrap_or(now),
            });
        }
        if let Some(caps) = limit_re.captures(text) {
            if let Some(n_str) = caps.get(1) {
                if let Ok(n) = n_str.as_str().parse::<u32>() {
                    results.push(StationTagInfo {
                        tag: StationTag::FuelLimit(n),
                        source: TagSource::Comment,
                        updated_at: created.unwrap_or(now),
                    });
                }
            }
        }
    }

    results
}

/// Дедупликация тегов: для каждого типа оставляем только один,
/// с наивысшим приоритетом источника (Api > Comment > Manual)
/// и наиболее свежим updated_at в пределах одного источника.
fn dedup_tags(tags: &mut Vec<StationTagInfo>) {
    let mut seen: Vec<StationTag> = Vec::new();
    let mut i = 0;
    while i < tags.len() {
        if let Some(pos) = seen.iter().position(|t| *t == tags[i].tag) {
            let existing = &tags[pos];
            let priority = |s: TagSource| -> u8 {
                match s {
                    TagSource::Api => 2,
                    TagSource::Comment => 1,
                    TagSource::Manual => 0,
                }
            };
            if priority(tags[i].source) > priority(existing.source)
                || (tags[i].source == existing.source
                    && tags[i].updated_at > existing.updated_at)
            {
                tags[pos] = tags.swap_remove(i);
            } else {
                tags.swap_remove(i);
            }
        } else {
            seen.push(tags[i].tag.clone());
            i += 1;
        }
    }
}

/// Возвращает текущее время UTC.
fn now_utc() -> DateTime<Utc> {
    Utc::now()
}

/// Преобразование DTO станции в модель Station.
/// Включает цены, теги (API + комментарии), общий статус.
pub fn station_from_dto(dto: BenzEstStationDto) -> Station {
    let name = dto.name.unwrap_or_default();
    let brand = if name.is_empty() {
        Brand::Unknown(String::new())
    } else {
        map_brand(&name)
    };

    // Преобразование DTO топлива в FuelAvailability.
    // checked_at берётся из price_updated_at (ISO 8601) — именно это поле
    // используется UnifiedProvider для определения свежести данных.
    let fuels: Vec<FuelAvailability> = dto.fuels.into_iter().map(fuel_from_dto).collect();

    // Сбор тегов из API
    let mut tags: Vec<StationTagInfo> = Vec::new();
    if let Some(ref api_tags) = dto.tags {
        for t in api_tags {
            if let Some(tag) = map_tag(&t.tag, t.value.as_deref()) {
                tags.push(StationTagInfo {
                    tag,
                    source: TagSource::Api,
                    updated_at: parse_tag_time(t.updated_at.as_deref()).unwrap_or_else(now_utc),
                });
            }
        }
    }

    // Сбор тегов из комментариев (очереди, лимиты)
    if let Some(ref comments) = dto.comments {
        let comment_tags = scan_comments_for_tags(comments);
        tags.extend(comment_tags);
    }

    // Удаление дубликатов тегов (Api приоритетнее Comment)
    dedup_tags(&mut tags);

    // Парсинг времени последнего обновления станции
    let last_updated = dto
        .last_updated
        .as_deref()
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc));

    Station {
        id: canonical_station_id(&dto.id),
        osm_id: dto.id,
        name,
        brand,
        address: dto.address.unwrap_or_default(),
        latitude: dto.lat,
        longitude: dto.lon,
        fuels,
        tags,
        overall_status: map_overall_status(dto.status.as_deref()),
        last_updated,
        reports_24h: dto.reports_24h,
    }
}

/// Преобразование DTO топлива в FuelAvailability.
/// checked_at = price_updated_at (реальное время обновления цены),
/// позволяет UnifiedProvider корректно сравнивать свежесть данных
/// между разными источниками.
fn fuel_from_dto(dto: BenzEstFuelDto) -> FuelAvailability {
    let checked_at = dto
        .price_updated_at
        .as_deref()
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(now_utc);

    FuelAvailability {
        fuel_type: map_fuel_type(&dto.fuel_type),
        status: map_fuel_status(&dto.status),
        provider: Provider::BenzEst,
        checked_at,
        price: dto.price.map(|p| p as f32),
    }
}
