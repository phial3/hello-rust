use crate::proto::interface::Executor;

pub struct RedirectExecutor {}

impl RedirectExecutor {
    pub fn new() -> Self {
        RedirectExecutor {}
    }
}

impl Executor for RedirectExecutor {}
