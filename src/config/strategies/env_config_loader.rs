use std::env;

use crate::config::{
    errors::PartialConfigLoadError,
    models::{ApiBind, LogLevel, PartialConfig},
    traits::PartialConfigLoader,
};

pub struct EnvConfigLoader;

impl EnvConfigLoader {}

impl PartialConfigLoader for EnvConfigLoader {
    fn load_partial_config() -> Result<PartialConfig, PartialConfigLoadError> {
        let log_level = env::var("IEMANJA_LOG_LEVEL")
            .ok()
            .as_deref()
            .and_then(|level| LogLevel::try_from(level).ok());

        let api_bind = env::var("IEMANJA_ADDRESS")
            .ok()
            .as_deref()
            .map(ApiBind::from);

        let db_address = env::var("IEMANJA_DATABASE").ok();

        Ok(PartialConfig {
            log_level,
            api_bind,
            db_address,
        })
    }
}
