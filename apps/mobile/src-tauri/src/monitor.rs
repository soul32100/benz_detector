use std::collections::HashMap;

use benz_models::fuel_status::FuelStatus;
use benz_models::fuel_type::FuelType;
use benz_models::station::Station;
use benz_models::station_tag::StationTagInfo;
use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

// ── События изменений ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct StationChangeEvent {
    pub station_id: String,
    pub station_name: String,
    pub station_address: String,
    pub fuel_changes: Vec<FuelChangeEvent>,
    pub tag_changes: Vec<TagChangeEvent>,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct FuelChangeEvent {
    pub fuel_type: String,
    pub change_type: String,
    pub old_status: Option<String>,
    pub new_status: String,
    pub old_price: Option<f32>,
    pub new_price: Option<f32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TagChangeEvent {
    pub tag: String,
    pub change_type: String,
}

// ── Внутреннее состояние ────────────────────────────────────────────────────

type FuelState = HashMap<Uuid, HashMap<String, (FuelStatus, Option<f32>)>>;
type TagState = HashMap<Uuid, Vec<StationTagInfo>>;

pub struct MonitorState {
    prev_fuels: FuelState,
    prev_tags: TagState,
    first: bool,
}

impl MonitorState {
    pub fn new() -> Self {
        Self {
            prev_fuels: HashMap::new(),
            prev_tags: HashMap::new(),
            first: true,
        }
    }

    pub fn check_station(
        &mut self,
        station: &Station,
    ) -> StationChangeEvent {
        let now = Utc::now().format("%H:%M:%S").to_string();
        let name = station_display_name(station);
        let addr = if station.address.is_empty() {
            String::new()
        } else {
            station.address.clone()
        };

        let mut fuel_changes = Vec::new();
        let mut tag_changes = Vec::new();

        if !self.first {
            fuel_changes = detect_fuel_changes(station, &self.prev_fuels);
            tag_changes = detect_tag_changes(station, &self.prev_tags);
        }

        let mut fuel_map = HashMap::new();
        for f in &station.fuels {
            fuel_map.insert(fuel_label(&f.fuel_type).to_string(), (f.status, f.price));
        }
        self.prev_fuels.insert(station.id, fuel_map);
        self.prev_tags.insert(station.id, station.tags.clone());

        StationChangeEvent {
            station_id: station.id.to_string(),
            station_name: name,
            station_address: addr,
            fuel_changes,
            tag_changes,
            timestamp: now,
        }
    }

    pub fn set_first_done(&mut self) {
        self.first = false;
    }

    pub fn is_first(&self) -> bool {
        self.first
    }
}

// ── Детекция изменений ─────────────────────────────────────────────────────

fn detect_fuel_changes(
    station: &Station,
    prev_fuels: &FuelState,
) -> Vec<FuelChangeEvent> {
    let mut changes = Vec::new();

    let old_fuels = match prev_fuels.get(&station.id) {
        Some(f) => f,
        None => return changes,
    };

    for fuel in &station.fuels {
        let label = fuel_label(&fuel.fuel_type);

        match old_fuels.get(label) {
            None => {
                if fuel.status == FuelStatus::Available {
                    changes.push(FuelChangeEvent {
                        fuel_type: label.to_string(),
                        change_type: "appeared".into(),
                        old_status: None,
                        new_status: "Available".into(),
                        old_price: None,
                        new_price: fuel.price,
                    });
                }
            }
            Some((old_status, old_price)) => {
                if old_status != &fuel.status {
                    let change_type = match fuel.status {
                        FuelStatus::Available => "appeared",
                        FuelStatus::Unavailable => "disappeared",
                        _ => "status_changed",
                    };
                    changes.push(FuelChangeEvent {
                        fuel_type: label.to_string(),
                        change_type: change_type.into(),
                        old_status: Some(format!("{old_status:?}")),
                        new_status: format!("{:?}", fuel.status),
                        old_price: *old_price,
                        new_price: fuel.price,
                    });
                } else if *old_status == FuelStatus::Available
                    && fuel.status == FuelStatus::Available
                {
                    if let (Some(op), Some(np)) = (old_price, fuel.price) {
                        if (op * 100.0).round() != (np * 100.0).round() {
                            changes.push(FuelChangeEvent {
                                fuel_type: label.to_string(),
                                change_type: "price_changed".into(),
                                old_status: None,
                                new_status: "Available".into(),
                                old_price: Some(*op),
                                new_price: Some(np),
                            });
                        }
                    }
                }
            }
        }
    }

    for (label, (old_status, _)) in old_fuels {
        let still_present = station.fuels.iter().any(|f| fuel_label(&f.fuel_type) == label);
        if !still_present && *old_status == FuelStatus::Available {
            changes.push(FuelChangeEvent {
                fuel_type: label.to_string(),
                change_type: "disappeared".into(),
                old_status: Some("Available".into()),
                new_status: "Unavailable".into(),
                old_price: None,
                new_price: None,
            });
        }
    }

    changes
}

fn detect_tag_changes(
    station: &Station,
    prev_tags: &TagState,
) -> Vec<TagChangeEvent> {
    let mut changes = Vec::new();

    let old_tags = match prev_tags.get(&station.id) {
        Some(t) => t,
        None => return changes,
    };

    for new_tag in &station.tags {
        let old = old_tags.iter().find(|t| t.tag == new_tag.tag);
        if old.is_none() {
            changes.push(TagChangeEvent {
                tag: format!("{:?}", new_tag.tag),
                change_type: "added".into(),
            });
        }
    }

    for old_tag in old_tags {
        let still_present = station.tags.iter().any(|t| t.tag == old_tag.tag);
        if !still_present {
            changes.push(TagChangeEvent {
                tag: format!("{:?}", old_tag.tag),
                change_type: "removed".into(),
            });
        }
    }

    changes
}

// ── Вспомогательное ─────────────────────────────────────────────────────────

fn station_display_name(s: &Station) -> String {
    let brand = match &s.brand {
        benz_models::brand::Brand::Unknown(b) if b.is_empty() => "—".to_string(),
        benz_models::brand::Brand::Unknown(b) => b.clone(),
        _ => format!("{:?}", s.brand),
    };
    if s.name.is_empty() {
        brand
    } else {
        s.name.clone()
    }
}

fn fuel_label(ft: &FuelType) -> &'static str {
    match ft {
        FuelType::Ai92 => "92",
        FuelType::Ai95 => "95",
        FuelType::Ai95Puls => "95+",
        FuelType::Diesel => "ДТ",
        FuelType::Ai98 => "98",
        FuelType::Ai100 => "100",
        FuelType::Gas => "Газ",
        FuelType::Unknown => "?",
    }
}
