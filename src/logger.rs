use crate::config::models::LogLevel;
use tracing_subscriber::EnvFilter;

impl From<&LogLevel> for tracing::Level {
    fn from(value: &LogLevel) -> Self {
        match value {
            LogLevel::Trace => tracing::Level::TRACE,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Error => tracing::Level::ERROR,
        }
    }
}

pub fn initialize_logger(log_level: &LogLevel) {
    let log_level = tracing::Level::from(log_level);
    let filter = EnvFilter::from_default_env()
        .add_directive("none".parse().unwrap())
        .add_directive(
            format!("{}={}", env!("CARGO_PKG_NAME"), log_level)
                .parse()
                .unwrap(),
        );

    let subscriber = tracing_subscriber::fmt().with_env_filter(filter).finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Unable to set global default subscriber");
}
