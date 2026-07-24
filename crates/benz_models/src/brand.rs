use serde::{Deserialize, Serialize};

/// название заправок
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Brand {
    Lukoil,
    Gazprom,
    Rosneft,
    Bashneft,
    Tatneft,
    Unknown(String),
}
