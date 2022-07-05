use crate::config::config_model::User;
use mysql_common::frunk::labelled::chars::u;
use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::sync::RwLock;

pub trait TenantManager {
    // GetTenants returns all tenants.
    fn get_tenants(&self) -> Vec<String>;
    // GetUser returns user by tenant and username.
    fn get_user(&self, tenant: String, username: String) -> Option<User>;
    // GetClusters returns cluster names.
    fn get_clusters(&self, tenant: String) -> Option<Vec<String>>;
    // GetTenantOfCluster returns the tenant of cluster.
    fn get_tenant_cluster(&self, cluster: String) -> Option<String>;
    // PutUser puts a user into tenant.
    fn put_user(&self, tenant: String, user: crate::config::config_model::User);
    // RemoveUser removes a user from tenant.
    fn remove_user(&self, tenant: String, username: String);
    // PutCluster puts a cluster into tenant.
    fn put_cluster(&self, tenant: String, cluster: String);
    // RemoveCluster removes a cluster from tenant.
    fn remove_cluster(&self, tenant: String, cluster: String);
}

#[derive(Debug, Clone)]
pub struct Tenant {
    cluster: HashMap<String, String>,
    users: HashMap<String, User>,
}

pub struct TenantManagerProvider {
    tenants: RwLock<HashMap<String, Tenant>>,
}

impl TenantManagerProvider {
    pub fn new() -> TenantManagerProvider {
        TenantManagerProvider {
            tenants: RwLock::new(HashMap::new()),
        }
    }
}

impl TenantManager for TenantManagerProvider {
    fn get_tenants(&self) -> Vec<String> {
        let tenants_map = &*(self.tenants.read().unwrap());
        let mut result: Vec<String> = Vec::new();
        for (k, _) in tenants_map {
            result.push(k.clone());
        }
        result
    }

    fn get_user(&self, tenant: String, username: String) -> Option<User> {
        let tenants_map = self.tenants.read().unwrap();
        if tenants_map.contains_key(tenant.as_str()) {
            return None;
        }
        let tenant = tenants_map.get(tenant.as_str()).unwrap();
        let user = tenant.users.get(username.as_str()).unwrap();
        Some(user.to_owned())
    }

    fn get_clusters(&self, tenant: String) -> Option<Vec<String>> {
        let tenants_map = self.tenants.read().unwrap();
        if tenants_map.contains_key(tenant.as_str()) {
            return None;
        }
        let tenant = tenants_map.get(tenant.as_str()).unwrap();
        let mut result: Vec<String> = Vec::new();
        for (k, _) in tenant.clone().cluster {
            result.push(k);
        }
        Some(result)
    }

    fn get_tenant_cluster(&self, cluster: String) -> Option<String> {
        let tenants_map = &*self.tenants.read().unwrap();
        for (_, tenant) in tenants_map {
            if tenant.cluster.contains_key(cluster.as_str()) {
                let c = tenant.cluster.get(cluster.as_str()).unwrap();
                return Some(c.to_string());
            }
        }
        None
    }

    fn put_user(&self, tenant: String, user: User) {
        let mut tenants_map = self.tenants.write().unwrap();
        let mut current = match tenants_map.get_mut(tenant.as_str()) {
            Some(t) => t,
            None => {
                let new_tenant = Tenant {
                    cluster: HashMap::new(),
                    users: HashMap::new(),
                };
                tenants_map.insert(tenant.clone(), new_tenant);
                tenants_map.get_mut(tenant.as_str()).unwrap()
            }
        };
        let users = &mut current.users;
        users.insert(user.username.clone(), user);
    }

    fn remove_user(&self, tenant: String, username: String) {
        let mut tenant_map = self.tenants.write().unwrap();
        if !tenant_map.contains_key(tenant.as_str()) {
            return;
        }
        let tenant = tenant_map.get_mut(tenant.as_str()).unwrap();
        if !tenant.users.contains_key(username.as_str()) {
            return;
        }
        tenant.users.remove(username.as_str());
    }

    fn put_cluster(&self, tenant: String, cluster: String) {
        let mut tenants_map = self.tenants.write().unwrap();
        let current = match tenants_map.get_mut(tenant.as_str()) {
            Some(t) => t,
            None => {
                let new_tenant = Tenant {
                    cluster: HashMap::new(),
                    users: HashMap::new(),
                };
                tenants_map.insert(tenant.clone(), new_tenant);
                tenants_map.get_mut(tenant.as_str()).unwrap()
            }
        };
        let mut clusters = &mut current.cluster;
        clusters.insert(cluster, String::new());
    }

    fn remove_cluster(&self, tenant: String, cluster: String) {
        let mut tenant_map = self.tenants.write().unwrap();
        if !tenant_map.contains_key(tenant.as_str()) {
            return;
        }
        let tenant = tenant_map.get_mut(tenant.as_str()).unwrap();
        if !tenant.cluster.contains_key(cluster.as_str()) {
            return;
        }
        tenant.cluster.remove(cluster.as_str());
    }
}
