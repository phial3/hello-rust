
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


fn type_convert() {
    let a = 3.1 as i8;
    let b = 100_i8 as i32;
    let c = 'a' as u8; // 将字符'a'转换为整数，97

    println!("{},{},{}",a,b,c);

    let b: i16 = 1500;

    let _b_: u8 = match b.try_into() {
        Ok(b1) => b1,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        }
    };
}