use crate::frontend;

pub type ProxyResult<T> = std::result::Result<T, ProxyError>;

#[derive(Debug)]
pub enum ProxyError {
    IO(std::io::Error),
    Other(Box<dyn std::error::Error + Send + Sync>),
}

impl From<std::io::Error> for ProxyError {
    fn from(e: std::io::Error) -> Self {
        ProxyError::IO(e)
    }
}

impl From<frontend::errors::FrontendError> for ProxyError {
    fn from(e: frontend::errors::FrontendError) -> Self {
        ProxyError::Other(Box::new(e))
    }
}

// impl From<router::RouterError> for ProxyError {
//     fn from(e: router::RouterError) -> Self {
//         ProxyError::Other(Box::new(e))
//     }
// }

impl std::error::Error for ProxyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ProxyError::IO(e) => e.source(),
            ProxyError::Other(e) => e.source(),
        }
    }
}

impl std::fmt::Display for ProxyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProxyError::IO(e) => e.fmt(f),
            ProxyError::Other(e) => e.fmt(f),
        }
    }
}
