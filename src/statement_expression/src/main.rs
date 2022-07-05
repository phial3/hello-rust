fn main() {
    add_with_extra(1, 2);

    let y = {
        let x = 3;
        x + 1
    };

    assert_eq!(y, 4);

    println!("The value of y is: {}", y);

    let v = {
        let mut x = 1;
        x += 2;
        x
    };
 
    assert_eq!(v, 3);
}

fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式
}
