use crate::config::config_model::Configuration;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigOptions {
    #[serde(rename(deserialize = "name"))]
    pub store_name: String,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Center {
    initialize: i32,
    // store_operate: StoreOperate,
    // conf_holder: atomic.Value,
    // lock:         sync.RWMutex
    // observers    []Observer
    // watchCancels []context.CancelFunc
}

impl Center {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // TODO
        let center = Center { initialize: 0 };
        return Ok(center);
    }

    pub fn load(&self) -> Result<Configuration, Box<dyn Error>> {
        let file =
            File::open("/Users/dongzonglei/source_code/Github/arana-rust/src/conf/config.yaml")?;
        //TODO is valid yaml file.
        let content = serde_yaml::from_reader(file);
        let configuration: Configuration = match content {
            Ok(content) => content,
            Err(err) => return Err(Box::new(err)),
        };
        Ok(configuration)
    }
}
