use async_trait::async_trait;

use crate::{error::LocationError, models::Location};

#[async_trait]
pub trait LocationProvider: Send + Sync {
    async fn current_location(&self) -> Result<Location, LocationError>;
}
