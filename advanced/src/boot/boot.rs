use std::error::Error;

use crate::boot::discovery::{Discovery, DiscoveryProvider};
use crate::config::Tenant;
use crate::security;
use crate::security::tenant::TenantManager;

pub fn bootstrap(config: String) -> Result<DiscoveryProvider, Box<dyn Error>> {
    let mut provider = DiscoveryProvider::new(config);
    match provider.init() {
        Some(err) => return Err(err),
        None => {}
    };
    let clusters = match provider.list_clusters() {
        Ok(cluster) => cluster,
        Err(err) => return Err(err),
    };
    for cluster in clusters {
        let _cluster = match provider.cluster(cluster){
            Ok(c) => c,
            Err(_) => continue,
        };
    }
    let tenants = match provider.list_tenants() {
        Ok(tenants) => tenants,
        Err(err) => return Err(err),
    };

    for item in tenants {
        let tenant = match provider.tenant(item.clone()) {
            Ok(tenant) => tenant,
            Err(err) => continue,
        };
        let tenant_provider = security::TenantManagerProvider::new();
        for user in tenant.users {
            tenant_provider.put_user(item.clone(), user);
        }
    };
    Ok(provider)
}

fn build_namespace() {
    todo!()
}
