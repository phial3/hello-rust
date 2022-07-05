use crate::mysql;

pub type FrontendResult<T> = std::result::Result<T, FrontendError>;

#[derive(Debug)]
pub enum FrontendError {
    IO(std::io::Error),
    MySQLErr(mysql::errors::MySQLError),
    ProxyAuthDenied,
    ProxyAuthOldInClientProtocol41,
}

impl From<std::io::Error> for FrontendError {
    fn from(e: std::io::Error) -> Self {
        FrontendError::IO(e)
    }
}

impl From<mysql::errors::MySQLError> for FrontendError {
    fn from(e: mysql::errors::MySQLError) -> Self {
        FrontendError::MySQLErr(e)
    }
}

impl std::error::Error for FrontendError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FrontendError::ProxyAuthDenied => None,
            FrontendError::ProxyAuthOldInClientProtocol41 => None,
            FrontendError::IO(e) => e.source(),
            FrontendError::MySQLErr(e) => e.source(),
        }
    }
}
impl std::fmt::Display for FrontendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrontendError::ProxyAuthDenied => write!(f, "proxy auth denied!"),
            FrontendError::ProxyAuthOldInClientProtocol41 => {
                write!(f, "Too old than CapabilityFlags::CLIENT_PROTOCOL_41!")
            }
            FrontendError::IO(e) => e.fmt(f),
            FrontendError::MySQLErr(e) => e.fmt(f),
        }
    }
}
