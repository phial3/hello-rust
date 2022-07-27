use std::fmt::{Debug, Display};

fn main() {
    println!("Hello, world!");
}

struct Foo<T: Display> {
    bar: T,
}

// struct Foo<T> where T:Display{
//     bar:T
// }

trait Eatable {
    fn eat(&self);
}

#[derive(Debug)]
struct Food<T>(T);

impl<T: Debug> Eatable for Food<T> {
    fn eat(&self) {
        println!("Eating{:?}", self);
    }
}

// impl<T> Eatable for Food<T>
// where
//     T: Debug,
// {
//     fn eat(&self) {
//         println!("Eating{:?}", self);
//     }
// }

trait Eat {
    fn eat(&self) {
        println!("eat");
    }
}
trait Code {
    fn code(&self) {
        println!("code");
    }
}
trait Sleep {
    fn sleep(&self) {
        println!("sleep");
    }
}

fn coder_life<T: Eat + Code + Sleep>(coder: T) {
    coder.eat();
    coder.code();
    coder.sleep();
}

fn lazy_adder(a: u32, b: u32) -> impl Fn() -> u32 {
    move || a + b
}
