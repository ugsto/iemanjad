use super::errors::ConfigLoadError;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub enum ApiBind {
    UnixSocket(String),
    Tcp(SocketAddr),
}

#[derive(Debug)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug)]
pub struct Config {
    pub log_level: LogLevel,
    pub api_bind: ApiBind,
    pub db_address: String,
}

#[derive(Default, Debug)]
pub struct PartialConfig {
    pub log_level: Option<LogLevel>,
    pub api_bind: Option<ApiBind>,
    pub db_address: Option<String>,
}

impl TryFrom<PartialConfig> for Config {
    type Error = ConfigLoadError;

    fn try_from(partial_config: PartialConfig) -> Result<Self, Self::Error> {
        let log_level = partial_config
            .log_level
            .ok_or(ConfigLoadError::MissingProperty("log_level"))?;
        let api_bind = partial_config
            .api_bind
            .ok_or(ConfigLoadError::MissingProperty("api_bind"))?;
        let db_address = partial_config
            .db_address
            .ok_or(ConfigLoadError::MissingProperty("db_address"))?;

        Ok(Self {
            log_level,
            api_bind,
            db_address,
        })
    }
}

impl PartialConfig {
    pub fn merge(self, other: PartialConfig) -> Self {
        Self {
            log_level: self.log_level.or(other.log_level),
            api_bind: self.api_bind.or(other.api_bind),
            db_address: self.db_address.or(other.db_address),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_config_to_config_success() {
        let log_level = LogLevel::Info;
        let api_bind = ApiBind::Tcp("127.0.0.1:8080".parse().unwrap());
        let db_address = "foobar".to_string();

        let partial_config = PartialConfig {
            log_level: Some(log_level),
            api_bind: Some(api_bind),
            db_address: Some(db_address),
        };

        let config = Config::try_from(partial_config).unwrap();

        assert!(
            matches!(config.api_bind, ApiBind::Tcp(addr) if addr == "127.0.0.1:8080".parse().unwrap())
        );
        assert_eq!(config.db_address, "foobar");
    }

    #[test]
    fn test_partial_config_missing_log_level() {
        let partial_config = PartialConfig {
            log_level: None,
            api_bind: Some(ApiBind::Tcp("127.0.0.1:8080".parse().unwrap())),
            db_address: Some("foobar".to_string()),
        };

        let result = Config::try_from(partial_config);
        assert!(
            matches!(result, Err(ConfigLoadError::MissingProperty(prop)) if prop == "log_level")
        );
    }

    #[test]
    fn test_partial_config_missing_api_bind() {
        let partial_config = PartialConfig {
            log_level: Some(LogLevel::Info),
            api_bind: None,
            db_address: Some("foobar".to_string()),
        };

        let result = Config::try_from(partial_config);
        assert!(
            matches!(result, Err(ConfigLoadError::MissingProperty(prop)) if prop == "api_bind")
        );
    }

    #[test]
    fn test_partial_config_missing_db_address() {
        let partial_config = PartialConfig {
            log_level: Some(LogLevel::Info),
            api_bind: Some(ApiBind::Tcp("127.0.0.1:8080".parse().unwrap())),
            db_address: None,
        };

        let result = Config::try_from(partial_config);
        assert!(
            matches!(result, Err(ConfigLoadError::MissingProperty(prop)) if prop == "db_address")
        );
    }

    #[test]
    fn test_partial_config_merge() {
        let log_level_1 = LogLevel::Info;
        let api_bind_1 = ApiBind::Tcp("127.0.0.1:8080".parse().unwrap());
        let db_address_2 = "foobar".to_string();

        let partial_config_1 = PartialConfig {
            log_level: Some(log_level_1),
            api_bind: Some(api_bind_1),
            db_address: None,
        };

        let partial_config_2 = PartialConfig {
            log_level: None,
            api_bind: None,
            db_address: Some(db_address_2.clone()),
        };

        let merged_config = partial_config_1.merge(partial_config_2);

        assert!(matches!(merged_config.log_level, Some(LogLevel::Info)));
        assert!(matches!(merged_config.api_bind, Some(ApiBind::Tcp(_))));
        assert_eq!(merged_config.db_address, Some(db_address_2));
    }
}
