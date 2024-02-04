use super::{errors::PartialConfigLoadError, models::PartialConfig};

pub trait PartialConfigLoader {
    fn load_partial_config() -> Result<PartialConfig, PartialConfigLoadError>;
}
