mod monitor;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use benz_core::provider::FuelProvider;
use benz_core::unified_provider::UnifiedProvider;
use benz_location::geocoding;
use benz_models::station::Station;
use benz_parser_benzest::BenzEstProvider;
use benz_parser_gdebenz::GdeBenzProvider;
use monitor::MonitorState;
use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tokio::task::JoinHandle;
use uuid::Uuid;

// ── Состояние приложения ─────────────────────────────────────────────────────

pub struct AppState {
    pub http_client: reqwest::Client,
    pub monitor_handle: Mutex<Option<JoinHandle<()>>>,
    pub stop_flag: Arc<AtomicBool>,
}

// ── DTO для фронтенда ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct LocationDto {
    pub latitude: f64,
    pub longitude: f64,
    pub city: Option<String>,
    pub region: Option<String>,
}

// ── Tauri команды ────────────────────────────────────────────────────────────

#[tauri::command]
async fn fetch_stations(
    state: tauri::State<'_, AppState>,
    lat: f64,
    lon: f64,
    radius_km: u32,
) -> Result<Vec<Station>, String> {
    let client = &state.http_client;
    let benzest = Box::new(BenzEstProvider::new(client.clone(), lat, lon, radius_km));
    let gdebenz = Box::new(GdeBenzProvider::new(client.clone(), lat, lon, radius_km));
    let provider = UnifiedProvider::new(vec![benzest, gdebenz]);

    provider.fetch_stations().await.map_err(|e| format!("{e:?}"))
}

#[tauri::command]
async fn search_city(
    state: tauri::State<'_, AppState>,
    query: String,
) -> Result<LocationDto, String> {
    let client = &state.http_client;
    match geocoding::search_city(client, &query).await {
        Ok(loc) => Ok(LocationDto {
            latitude: loc.latitude,
            longitude: loc.longitude,
            city: loc.city,
            region: loc.region,
        }),
        Err(e) => Err(format!("{e}")),
    }
}

#[tauri::command]
async fn start_monitor(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    station_ids: Vec<String>,
    interval_minutes: u64,
) -> Result<(), String> {
    let mut handle = state.monitor_handle.lock().map_err(|e| e.to_string())?;

    if handle.is_some() {
        return Err("Мониторинг уже запущен".into());
    }

    let stop_flag = Arc::new(AtomicBool::new(false));
    state.stop_flag.store(false, Ordering::SeqCst);

    let client = reqwest::Client::new();

    let ids: Vec<Uuid> = station_ids
        .iter()
        .filter_map(|s| Uuid::parse_str(s).ok())
        .collect();

    let handle_clone = app.clone();
    let stop = stop_flag.clone();

    let task = tokio::spawn(async move {
        let mut monitor_state = MonitorState::new();
        let http = client;

        loop {
            if stop.load(Ordering::SeqCst) {
                break;
            }

            match fetch_all_stations(&http).await {
                Ok(all) => {
                    let tracked: Vec<&Station> =
                        all.iter().filter(|s| ids.contains(&s.id)).collect();

                    if !monitor_state.is_first() {
                        let mut all_changes = Vec::new();
                        for station in &tracked {
                            let event = monitor_state.check_station(station);
                            if !event.fuel_changes.is_empty() || !event.tag_changes.is_empty() {
                                all_changes.push(event);
                            }
                        }
                        for change in all_changes {
                            let _ = handle_clone.emit("station-change", change);
                        }
                    } else {
                        for station in &tracked {
                            monitor_state.check_station(station);
                        }
                        monitor_state.set_first_done();
                    }
                }
                Err(e) => {
                    let _ = handle_clone.emit("monitor-error", format!("{e:?}"));
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(interval_minutes * 60)).await;
        }
    });

    *handle = Some(task);

    Ok(())
}

#[tauri::command]
async fn stop_monitor(state: tauri::State<'_, AppState>) -> Result<(), String> {
    state.stop_flag.store(true, Ordering::SeqCst);

    let mut handle = state.monitor_handle.lock().map_err(|e| e.to_string())?;
    if let Some(h) = handle.take() {
        h.abort();
    }

    Ok(())
}

// ── Внутренняя логика ───────────────────────────────────────────────────────

async fn fetch_all_stations(
    client: &reqwest::Client,
) -> Result<Vec<Station>, String> {
    let lat = 55.75;
    let lon = 37.62;
    let radius = 50;

    let benzest = Box::new(BenzEstProvider::new(client.clone(), lat, lon, radius));
    let gdebenz = Box::new(GdeBenzProvider::new(client.clone(), lat, lon, radius));
    let provider = UnifiedProvider::new(vec![benzest, gdebenz]);

    provider.fetch_stations().await.map_err(|e| format!("{e:?}"))
}

// ── Запуск приложения ────────────────────────────────────────────────────────

#[cfg(mobile)]
#[tauri::mobile_entry_point]
pub fn run() {
    let http_client = reqwest::Client::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_geolocation::init())
        .plugin(tauri_plugin_notification::init())
        .manage(AppState {
            http_client,
            monitor_handle: Mutex::new(None),
            stop_flag: Arc::new(AtomicBool::new(false)),
        })
        .invoke_handler(tauri::generate_handler![
            fetch_stations,
            search_city,
            start_monitor,
            stop_monitor,
        ])
        .run(tauri::generate_context!())
        .expect("Ошибка запуска Tauri приложения");
}
