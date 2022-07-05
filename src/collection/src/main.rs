use std::error::Error;

#[derive(Debug)]
pub struct A {
    pub name: String,
    pub b: B,
}

#[derive(Debug)]
pub struct B {
    pub age: u32,
    pub add: String,
}

fn main() {
    match list_add() {
        Ok(data) => println!("{:?}", data),
        Err(err) => println!("{:?}", err),
    }
}

fn list_add() -> Result<Vec<String>, Box<dyn Error>> {
    let mut collector: Vec<A> = Vec::new();
    collector.push(A{
        name: String::from("a1"),
        b: B{
            age: 16,
            add: String::from("add1"),
        }
    });
    collector.push(A{
        name: String::from("a2"),
        b: B{
            age: 16,
            add: String::from("add2"),
        }
    });
    let mut result: Vec<String> = Vec::new();
    let filter = collector.into_iter().find(|a|a.name==String::from("a"));
    if filter.is_none() {
        return Ok(result);
    }
    result.push(filter.unwrap().b.add);
    Ok(result)
}