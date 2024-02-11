use crate::config::{
    errors::PartialConfigLoadError,
    models::{ApiBind, LogLevel, PartialConfig},
    traits::PartialConfigLoader,
};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliConfigLoader {
    /// Log level: trace, debug, info, warn, error
    #[clap(long, default_value = "info")]
    pub log_level: Option<String>,

    /// API binding address, e.g., "127.0.0.1:7029" for TCP or "/tmp/api.sock" for Unix socket
    #[clap(long, default_value = "/tmp/iemanja.sock")]
    pub api_bind: Option<ApiBind>,

    /// Database address, e.g., "ws://127.0.0.1:8000" for external db or "speedb:///etc/iemanjad/iemanjad.surreal" for local
    #[clap(long, default_value = "speedb:///etc/iemanjad/iemanjad.surreal")]
    pub db_address: Option<String>,
}

impl PartialConfigLoader for CliConfigLoader {
    fn load_partial_config() -> Result<PartialConfig, PartialConfigLoadError> {
        let config = Self::parse();

        let log_level = config
            .log_level
            .as_deref()
            .and_then(|level| LogLevel::try_from(level).ok());
        let api_bind = config.api_bind;
        let db_address = config.db_address;

        Ok(PartialConfig {
            log_level,
            api_bind,
            db_address,
        })
    }
}
