/// Бренды сетей АЗС.

use serde::{Deserialize, Serialize};

/// Сеть АЗС по названию бренда.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Brand {
    Lukoil,
    Gazprom,
    Rosneft,
    Bashneft,
    Tatneft,
    /// Неизвестный или нераспознанный бренд (содержит сырое название).
    Unknown(String),
}
