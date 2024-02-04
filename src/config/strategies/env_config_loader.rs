use std::env;

use crate::config::{
    errors::PartialConfigLoadError,
    models::{ApiBind, PartialConfig},
    traits::PartialConfigLoader,
};

pub struct EnvConfigLoader;

impl EnvConfigLoader {}

impl PartialConfigLoader for EnvConfigLoader {
    fn load_partial_config() -> Result<PartialConfig, PartialConfigLoadError> {
        let api_bind = env::var("IEMANJA_ADDRESS")
            .ok()
            .as_deref()
            .map(ApiBind::from);

        let db_address = env::var("IEMANJA_DATABASE").ok();

        Ok(PartialConfig {
            api_bind,
            db_address,
        })
    }
}
