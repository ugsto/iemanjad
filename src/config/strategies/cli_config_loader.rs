use crate::config::{
    errors::PartialConfigLoadError,
    models::{ApiBind, PartialConfig},
    traits::PartialConfigLoader,
};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliConfigLoader {
    /// API binding address, e.g., "127.0.0.1:7029" for TCP or "/tmp/api.sock" for Unix socket
    #[clap(long, default_value = "127.0.0.1:7029")]
    pub api_bind: Option<String>,

    /// Database address, e.g., "ws://127.0.0.1:8000" for external db or "speedb:///etc/iemanjad/iemanjad.surreal" for local
    #[clap(long, default_value = "speedb:///etc/iemanjad/iemanjad.surreal")]
    pub db_address: Option<String>,
}

impl PartialConfigLoader for CliConfigLoader {
    fn load_partial_config() -> Result<PartialConfig, PartialConfigLoadError> {
        let config = Self::parse();

        let api_bind = config.api_bind.as_deref().map(ApiBind::from);
        let db_address = config.db_address;

        Ok(PartialConfig {
            api_bind,
            db_address,
        })
    }
}
