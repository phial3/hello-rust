use crate::proto::interface;
use crate::proto::interface::Executor;
use std::collections::HashMap;

pub struct ServerConfig {
    pub server_version: String,
}

pub struct Listener<T>
where
    T: Executor,
{
    config: ServerConfig,
    listener: String, // TODO
    executor: T,
    conn_id: u32,
    conn_read_buffer_size: u16,
    capabilities: u32,
    character_set: u8,
    schema_name: String,
    statement_id: u32,
    stmts: HashMap<String, String>,
}

impl <T: Executor>interface::Listener for Listener<T> {
    fn set_executor(&self, executor: Box<dyn Executor>) {
        todo!()
    }

    fn listen(&self) {
        todo!()
    }

    fn close(&self) {
        todo!()
    }
}

impl <T: Executor>Listener<T> {
    pub fn new(executor: T, config: crate::config::Listener) -> Self {
        let config = ServerConfig {
            server_version: config.server_version,
        };

        //TODO Listener

        Listener {
            config,
            listener: "".to_string(),
            executor,
            conn_id: 0,
            conn_read_buffer_size: 0,
            capabilities: 0,
            character_set: 0,
            schema_name: "".to_string(),
            statement_id: 0,
            stmts: Default::default()
        }
    }
}
