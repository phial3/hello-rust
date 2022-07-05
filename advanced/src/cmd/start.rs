use crate::boot::boot;
use crate::boot::discovery::{Discovery, DiscoveryProvider};
use crate::config::Filter;
use crate::executor::redirect::RedirectExecutor;
use crate::mysql::server::Listener;
use crate::proto::interface::{Executor, FilterFactory};
use crate::server::server::Server;

pub fn run(config: String) {
    //TODO default config.
    let provider = match boot::bootstrap(config) {
        Ok(provider) => provider,
        Err(_) => {
            // TODO log
            return;
        }
    };
    let filters = provider.list_filters();
    if filters.is_err() {
        // TODO log
        return;
    }
    for each in filters.unwrap() {
        let factory = each.factory(each.name.clone());
        if factory.is_none() {
            panic!()
        }
        let factory = factory.unwrap();
        let filter: Filter = match factory.new_filter(each.config.clone()) {
            Ok(filter) => filter,
            Err(err) => panic!(),
        };
        filter.register(filter.name.clone());
    }

    let listeners_config = match provider.list_listeners() {
        Ok(listeners) => listeners,
        Err(err) => {
            // TODO log
            return;
        }
    };

    let mut listeners: Vec<Box<dyn crate::proto::interface::Listener>> = Vec::new();
    for config in listeners_config {
        let executor = RedirectExecutor::new();
        let listener = Listener::new(executor, config);
        listeners.push(Box::new(listener));
    }
    let server = Server::new(listeners);
    server.start();
}
