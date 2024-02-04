use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigLoadError {
    #[error("Missing property: {0}")]
    MissingProperty(&'static str),
}

#[derive(Debug, Error)]
pub enum PartialConfigLoadError {
    #[error("Unsupported log level: {0}")]
    UnsupportedLogLevel(String),
}
