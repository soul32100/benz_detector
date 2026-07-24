use benz_parser_gdebenz::client::GdeBenzClient;

#[tokio::test]
async fn test_fetch_stations_from_stavropol() {
    let client = reqwest::Client::new();
    let gdebenz = GdeBenzClient::new(client);

    let stations = gdebenz
        .fetch_stations(45.00, 41.90, 45.10, 42.00)
        .await;

    match stations {
        Ok(list) => {
            assert!(!list.is_empty(), "Должны быть АЗС в Ставрополе");
            println!("Найдено станций: {}", list.len());
            for s in &list {
                println!(
                    "  osm_id={} name={:?} brand={:?} addr={:?} status={:?} fuels={:?}",
                    s.osm_id, s.name, s.brand, s.addr, s.status, s.fuels_now
                );
            }
        }
        Err(e) => {
            panic!("Ошибка API: {}", e);
        }
    }
}

#[tokio::test]
async fn test_fetch_nearby_stavropol() {
    let client = reqwest::Client::new();
    let gdebenz = GdeBenzClient::new(client);

    let stations = gdebenz.fetch_nearby(45.05, 41.95, 20).await;

    match stations {
        Ok(list) => {
            assert!(!list.is_empty(), "Должны быть АЗС рядом со Ставрополем");
            println!("Найдено рядом: {}", list.len());
            for s in &list {
                println!(
                    "  {} {} статус={:?} топливо={:?} расстояние={}км",
                    s.brand.as_deref().unwrap_or("?"),
                    s.addr.as_deref().unwrap_or("?"),
                    s.status,
                    s.fuels_now,
                    s.distance_km.map(|d| d.to_string()).unwrap_or_default()
                );
            }
        }
        Err(e) => {
            panic!("Ошибка API nearby: {}", e);
        }
    }
}
