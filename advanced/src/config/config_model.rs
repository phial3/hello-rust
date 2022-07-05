use regex::{Captures, Regex};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

use crate::proto::interface::FilterFactory;
use serde::Deserialize;

pub enum ProtocolType {
    MySQL(u8),
    HTTP(u8),
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub kind: String,
    #[serde(rename(serialize = "api_version", deserialize = "apiVersion"))]
    pub api_version: String,
    pub metadata: HashMap<String, String>,
    pub data: Data,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SocketAddress {
    pub address: String,
    pub port: u32,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub filters: Option<Vec<Filter>>,
    pub listeners: Option<Vec<Listener>>,
    pub tenants: Vec<Tenant>,
    pub clusters: Vec<DataSourceCluster>,
    pub sharding_rule: ShardingRule,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Filter {
    pub name: String,
    pub config: String,
}

impl Filter {
    pub fn factory(&self, name: String) -> Option<Box<dyn FilterFactory>> {
        todo!()
    }

    pub fn register(&self, name: String) {
        todo!()
    }
}

#[derive(Debug, Deserialize)]
pub struct Tenant {
    pub name: String,
    pub users: Vec<User>,
}

#[derive(Debug, Deserialize)]
pub struct DataSourceCluster {
    pub name: String,
    #[serde(rename(serialize = "data_source_type", deserialize = "type"))]
    pub data_source_type: String,
    pub sql_max_limit: i32,
    pub tenant: String,
    pub conn_props: ConnProp,
    pub groups: Vec<Group>,
}

#[derive(Debug, Deserialize)]
pub struct ConnProp {
    pub capacity: u32,
    pub max_capacity: u32,
    pub idle_timeout: u32,
}

#[derive(Debug, Deserialize)]
pub struct Group {
    pub name: String,
    pub nodes: Vec<Node>,
}

#[derive(Debug, Deserialize)]
pub struct Node {
    pub name: String,
    pub host: String,
    pub port: u32,
    pub username: String,
    pub password: String,
    pub database: String,
    pub conn_props: HashMap<String, String>,
    pub weight: String,
    pub labels: HashMap<String, String>,
}

impl Node {
    pub fn read_write_weight(&self) -> Result<ReadWriteWeight, Box<dyn Error>> {
        let weight_regex = Regex::new(r"^[rR]([0-9]+)[wW]([0-9]+)$").unwrap();
        let result: Vec<Captures> = weight_regex.captures_iter(&self.weight).collect();
        if result.is_empty() {
            // TODO error
        }
        let captures = &result[0];
        if captures.len() != 3 {
            // TODO err
        }
        let read = captures[1].to_string().parse::<u32>().unwrap();
        let write = captures[2].to_string().parse::<u32>().unwrap();
        let result = ReadWriteWeight { read, write };
        Ok(result)
    }
}

#[derive(Debug, Deserialize)]
pub struct ShardingRule {
    pub tables: Vec<Table>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Listener {
    pub protocol_type: String,
    pub socket_address: SocketAddress,
    pub server_version: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Table {
    pub name: String,
    pub allow_full_scan: bool,
    pub db_rules: Option<Vec<Rule>>,
    pub tbl_rules: Option<Vec<Rule>>,
    pub topology: Topology,
    pub shadow_topology: Option<Topology>,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub column: String,
    pub expr: String,
}

#[derive(Debug, Deserialize)]
pub struct Topology {
    pub db_pattern: String,
    pub tbl_pattern: String,
}

pub struct ReadWriteWeight {
    pub read: u32,
    pub write: u32,
}

pub fn load(config_path: String) -> Result<Configuration, Box<dyn Error>> {
    let file = match File::open(config_path.clone()) {
        Ok(file) => file,
        Err(err) => {
            panic!(
                "Open config file is error: {:?}, config path: {:?}",
                err, config_path
            )
        }
    };
    let content = serde_yaml::from_reader(file);
    let config: Configuration = match content {
        Ok(content) => content,
        Err(err) => {
            panic!(
                "Read config content is error: {:?}, config path: {:?}",
                err, config_path
            )
        }
    };
    Ok(config)
}

// pub fn protocol_type()

#[cfg(test)]
mod tests {
    use log::error;
    use regex::{Captures, Regex};

    use crate::config::config_model::load;

    #[test]
    fn load_config() {
        let config = load(String::from(
            "/Users/dongzonglei/source_code/Github/arana-rust/src/config/config.yaml",
        ));
        match config {
            Ok(content) => {
                println!("Load config content is: {:?}", content);
            }
            Err(e) => {
                println!("Load config is err: {:?}", e);
            }
        }
    }

    #[test]
    fn read_write_weight() {
        let weight_regex = Regex::new(r"^[rR]([0-9]+)[wW]([0-9]+)$").unwrap();
        let result: Vec<Captures> = weight_regex.captures_iter("r3w5").collect();
        if !result.is_empty() {
            let cap = &result[0];
            println!("Month: {} Day: {}", &cap[2], &cap[1]);
        }
        println!("Load config is err: {:?}", result);
    }
}
