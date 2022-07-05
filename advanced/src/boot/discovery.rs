use std::error::Error;
use std::fs::File;

use serde::Deserialize;
use crate::boot::error::BootstrapError;

use crate::config::{
    Center, Configuration, DataSourceCluster, Filter, Group, Listener, Node, Tenant,
};

pub trait Discovery {
    // Init init discovery
    fn init(&mut self) -> Option<Box<dyn Error>>;

    // ListTenants list tenants name
    fn list_tenants(&self) -> Result<Vec<String>, Box<dyn Error>>;

    // GetTenant returns the tenant info
    fn tenant(&self, name: String) -> Result<Tenant, Box<dyn Error>>;

    // ListListeners lists the listener names
    fn list_listeners(&self) -> Result<Vec<Listener>, Box<dyn Error>>;

    // ListFilters list the filter names
    fn list_filters(&self) -> Result<Vec<Filter>, Box<dyn Error>>;

    // ListClusters lists the cluster names.
    fn list_clusters(&self) -> Result<Vec<String>, Box<dyn Error>>;

    // GetCluster returns the cluster info
    fn cluster(&self, name: String) -> Result<DataSourceCluster, Box<dyn Error>>;

    // ListGroups lists the group names.
    fn list_groups(&self, cluster: String) -> Result<Vec<String>, Box<dyn Error>>;

    // ListNodes lists the node names.
    fn list_nodes(&self, cluster: String, group: String) -> Result<Vec<String>, Box<dyn Error>>;

    // GetNode returns the node info.
    fn node(
        &self,
        cluster: String,
        group: String,
        node: String,
    ) -> Result<Option<Node>, Box<dyn Error>>;

    // ListTables lists the table names.
    fn list_tables(&self, cluster: String) -> Result<Vec<String>, Box<dyn Error>>;

    // GetTable returns the table info.
    // TODO return Result<rule.VTable, Error>;
    fn table(&self, cluster: String, table: String) -> Result<Vec<String>, Box<dyn Error>>;

    // GetConfigCenter
    fn config_center(self) -> Result<Center, Box<dyn Error>>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct BootOptions {
    pub config: crate::config::ConfigOptions,
}

#[derive(Debug, Clone)]
pub struct DiscoveryProvider {
    pub path: String,
    pub options: Option<BootOptions>,
    pub center: Option<Center>,
}

impl DiscoveryProvider {
    pub fn new(path: String) -> Self {
        DiscoveryProvider {
            path,
            options: None,
            center: None,
        }
    }
    fn load_boot_options(&mut self) -> Option<Box<dyn Error>> {
        let file = match File::open(&self.path) {
            Ok(file) => file,
            Err(err) => return Some(Box::new(err)),
        };
        //TODO is valid yaml file.
        let content = serde_yaml::from_reader(file);
        let options: BootOptions = match content {
            Ok(content) => content,
            Err(err) => return Some(Box::new(err)),
        };
        self.options = Some(options);
        None
    }

    fn init_config_center(&mut self) -> Option<Box<dyn Error>> {
        let center = Center::new();
        let center = match center {
            Ok(data) => data,
            Err(err) => return Some(err),
        };
        self.center = Some(center);
        None
    }

    fn load(&self) -> Configuration {
        let center = match &self.center {
            Some(center) => center,
            None => panic!("Load config center is error"),
        };

        let config: Configuration = match center.load() {
            Ok(config) => config,
            Err(err) => panic!("Load config center err: {:?}", err),
        };
        config
    }
}

impl Discovery for DiscoveryProvider {
    fn init(&mut self) -> Option<Box<dyn Error>> {
        match self.load_boot_options() {
            Some(err) => {
                return Some(err)
            }
            None => {
                self.init_config_center()
            }
        }
    }

    fn list_tenants(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut tenants = Vec::new();
        for each in self.load().data.tenants {
            tenants.push(each.name);
        }
        Ok(tenants)
    }

    fn tenant(&self, name: String) -> Result<Tenant, Box<dyn Error>> {
        for each in self.load().data.tenants {
            if each.name == name {
                return Ok(each);
            }
        }
        Err(Box::new(BootstrapError::TenantNotExist(name)))
    }

    fn list_listeners(&self) -> Result<Vec<Listener>, Box<dyn Error>> {
        return match self.load().data.listeners {
            Some(listener) => Ok(listener),
            None => Ok(Vec::new()),
        };
    }

    fn list_filters(&self) -> Result<Vec<Filter>, Box<dyn Error>> {
        return match self.load().data.filters {
            Some(filters) => Ok(filters),
            None => Ok(Vec::new()),
        };
    }

    fn list_clusters(&self) -> Result<Vec<String>, Box<dyn Error>> {
        Ok(self
            .load()
            .data
            .clusters
            .iter()
            .map(|cluster| cluster.name.clone())
            .collect())
    }

    fn cluster(&self, name: String) -> Result<DataSourceCluster, Box<dyn Error>> {
        for each in self.load().data.clusters {
            if each.name == name {
                return Ok(each);
            }
        }
        Err(Box::new(BootstrapError::DataSourceClusterNotExist(name)))
    }

    fn list_groups(&self, cluster: String) -> Result<Vec<String>, Box<dyn Error>> {
        let cluster = self.cluster(cluster);
        let cluster = match cluster {
            Ok(cluster) => cluster,
            Err(err) => return Err(err),
        };

        Ok(cluster
            .groups
            .iter()
            .map(|group| group.name.clone())
            .collect())
    }

    fn list_nodes(&self, cluster: String, group: String) -> Result<Vec<String>, Box<dyn Error>> {
        let cluster = self.cluster(cluster);
        let cluster = match cluster {
            Ok(cluster) => cluster,
            Err(err) => return Err(err),
        };
        let group: Option<Group> = cluster.groups.into_iter().find(|g| g.name.eq(&group));
        let result = Vec::new();
        if group.is_none() {
            return Ok(result);
        }
        Ok(group
            .unwrap()
            .nodes
            .iter()
            .map(|node| node.name.clone())
            .collect())
    }

    fn node(
        &self,
        cluster: String,
        group: String,
        node: String,
    ) -> Result<Option<Node>, Box<dyn Error>> {
        let cluster = self.cluster(cluster);
        let cluster = match cluster {
            Ok(cluster) => cluster,
            Err(err) => return Err(err),
        };
        let group: Option<Group> = cluster.groups.into_iter().find(|g| g.name.eq(&group));
        if group.is_none() {
            return Ok(None);
        }
        Ok(group.unwrap().nodes.into_iter().find(|n| n.name.eq(&node)))
    }

    fn list_tables(&self, cluster: String) -> Result<Vec<String>, Box<dyn Error>> {
        todo!()
    }

    fn table(&self, cluster: String, table: String) -> Result<Vec<String>, Box<dyn Error>> {
        todo!()
    }

    fn config_center(self) -> Result<Center, Box<dyn Error>> {
        Ok(self.center.unwrap())
    }
}
