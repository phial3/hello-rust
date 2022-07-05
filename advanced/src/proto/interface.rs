use crate::config::Filter;
use std::error::Error;

pub trait FilterFactory {
    fn new_filter(&self, config: String) -> Result<Filter, Box<dyn Error>>;
}

pub trait Listener {
    fn set_executor(&self, executor: Box<dyn Executor>);

    fn listen(&self);

    fn close(&self);
}

// Executor
pub trait Executor {
    // TODO
}
