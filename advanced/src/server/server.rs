use std::error::Error;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use crate::proto::interface::Listener;
use crate::proxy::errors::ProxyResult;
use crate::{frontend, proxy, router};

pub struct Server {
    pub listeners: Vec<Box<dyn Listener>>,
}

impl Server {
    pub fn new(listeners: Vec<Box<dyn Listener>>) -> Self {
        return Server { listeners };
    }

    pub fn add_listener(mut self, listener: Box<dyn Listener>) {
        self.listeners.push(listener);
    }

    pub fn start(&self) {
        for each in &self.listeners {
            each.listen();
            // proxy::ProxyServer::new().run()?;
        }
    }
}
