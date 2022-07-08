
#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub param: String,
}

impl Command {
    pub fn new () -> Self {
        Command {
            name: String::from("test"),
            param: String::from("test_param"),
        }
    }

    fn a(&self) {
        println!("a print {:?}", self);
    }

    fn b(&self) {
        println!("b print {:?}", self);
    }
}

fn main() {
    let cmd = Command {
        name: String::from("test"),
        param: String::from("test_param"),
    };

    cmd.a();
    cmd.b();
}
