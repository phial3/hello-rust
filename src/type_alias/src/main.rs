
fn add(i: i32, j: i32) -> i32 {
    i + j
}

//type A = add;

type Num = i32;

#[derive(Debug)]
enum Color {
    RED,
    GREEN
}

type c = Color;

fn main() {
    println!("Hello, world!{}", add(1, 2));

    // error
    // let x: i32 = 5;
    // let y: i64 = 5;
    //
    // // error
    // if x == y {
    //
    // }

    let i: i32 = 2;
    let j: Num = 2;
    if i == j {

    }

    println!("{:?}", c::RED);

    let ref a: i32;
    a = &1;
}
