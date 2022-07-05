
pub trait Listener {
    fn listen(&self);
}

pub struct ListenerImpl {
    name: String,
}

impl Listener for ListenerImpl {
    fn listen(&self) {
        println!("{}", self.name);
    }
}

pub struct Server<T> where T: Listener {
    listeners: Vec<T>,
}

impl <T: Listener> Server<T> {
    pub fn new(listeners: Vec<T>) -> Self {
        return Server {
            listeners,
        }
    }
}

fn main() {
    let listener = ListenerImpl{name:String::from("Hello, Generics")};
    let mut listeners = Vec::new();
    listeners.push(listener);
    let server = Server::new(listeners);
    server.listeners[0].listen();
}
