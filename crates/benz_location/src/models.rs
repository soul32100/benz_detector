use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub country: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
}
