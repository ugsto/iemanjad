use std::{net::SocketAddr, path::PathBuf};

use super::{
    errors::PartialConfigLoadError,
    models::{ApiBind, LogLevel},
};

impl From<&str> for ApiBind {
    fn from(api_bind: &str) -> Self {
        if let Ok(api_bind_address) = api_bind.parse::<SocketAddr>() {
            return ApiBind::Tcp(api_bind_address);
        }

        let api_bind_path = PathBuf::from(api_bind);

        ApiBind::UnixSocket(api_bind_path.display().to_string())
    }
}

impl TryFrom<&str> for LogLevel {
    type Error = PartialConfigLoadError;

    fn try_from(log_level: &str) -> Result<Self, PartialConfigLoadError> {
        match log_level {
            "trace" => Ok(LogLevel::Trace),
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            _ => Err(PartialConfigLoadError::UnsupportedLogLevel(
                log_level.to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_str_to_api_bind_with_tcp() {
        let tcp_address = "127.0.0.1:8080";
        let expected = SocketAddr::from_str(tcp_address).unwrap();

        match ApiBind::from(tcp_address) {
            ApiBind::Tcp(addr) => assert_eq!(addr, expected),
            _ => panic!("Expected TCP address, got Unix socket"),
        }
    }

    #[test]
    fn test_str_to_api_bind_with_unix_socket() {
        let unix_socket_path = "/tmp/api.sock";

        match ApiBind::from(unix_socket_path) {
            ApiBind::UnixSocket(path) => assert_eq!(path, unix_socket_path),
            _ => panic!("Expected Unix socket, got TCP address"),
        }
    }
}
