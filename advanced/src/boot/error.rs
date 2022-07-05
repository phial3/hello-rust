#![allow(dead_code)]
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Clone)]
pub enum BootstrapError {
    DataSourceClusterNotExist(String),
    TenantNotExist(String),
}

impl std::fmt::Display for BootstrapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BootstrapError::DataSourceClusterNotExist(cluster) => write!(f, "BootstrapError::DataSourceClusterNotExist::clusster:{}", cluster),
            BootstrapError::TenantNotExist(tenant) => write!(f, "BootstrapError::TenantNotExist::Tenant:{}", tenant),
        }
    }
}

impl std::error::Error for BootstrapError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BootstrapError::DataSourceClusterNotExist(..) => None,
            BootstrapError::TenantNotExist(..) => None,
        }
    }
}
