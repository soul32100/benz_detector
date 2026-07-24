use serde::{Deserialize, Serialize};

/// Источник получения информации о наличии топлива.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Provider {
    GdeBenz,
    BenzEst,
}
