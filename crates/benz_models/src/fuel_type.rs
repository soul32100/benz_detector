use serde::{Deserialize, Serialize};

/// Тип топлива.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FuelType {
    Ai92,
    Ai95,
    Ai95Puls,
    Ai98,
    Ai100,
    Diesel,
    Gas,
    Unknown,
}
