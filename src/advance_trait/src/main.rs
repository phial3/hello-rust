use std::fmt::{Debug, Formatter};

#[derive(Debug)]
struct Graph {
    graphs: Vec<Box<dyn Area>>,
}

#[derive(Debug)]
struct Square(f32);

#[derive(Debug)]
struct Rectangle(f32, f32);

struct Circle<T> {
    radius: T,
}

trait Area: Debug {
    fn get_area(&self) -> f32;
}

impl Area for Square {
    fn get_area(&self) -> f32 {
        self.0 * self.0
    }
}

impl Area for Rectangle {
    fn get_area(&self) -> f32 {
        self.0 * self.1
    }
}

impl<i32> Debug for Circle<i32> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl <i32> Area for Circle<i32>{
    fn get_area(&self) -> f32 {
        //self.radius * self.radius * 3.14
        100.0
    }
}

fn list_trait() -> Option<Box<dyn Area>> {
    Some(Box::new(Square(3f32)))
}

fn list_trait2() -> Box<dyn Area> {
    Box::new(Rectangle(3f32, 4f32))
}

fn main() {
    let s: &dyn Area = &Square(3f32);
    println!("{:?}", s.get_area());
    let rec: &dyn Area = &Rectangle(4f32, 2f32);
    println!("{:?}", rec.get_area());

    let trait_test = list_trait().unwrap();
    println!("{:?}", trait_test.get_area());

    let trait_test2 = list_trait2();
    println!("{:?}", trait_test2.get_area());

    let mut graphs: Vec<Box<dyn Area>> = Vec::new();
    graphs.push(Box::new(Square(5f32)));
    graphs.push(Box::new(Rectangle(6f32, 7f32)));
    graphs.push(Box::new(Circle{radius: 10}));

    for each in graphs {
        println!("{:?}", each.get_area());
    }
}
