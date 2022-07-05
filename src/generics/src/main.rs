fn main() {
    println!("add i8: {}", add(2i8, 3i8));
    println!("add i32: {}", add(20, 30));
    println!("add f64: {}", add(1.23, 1.23));

    let arr: [i32; 3] = [1, 2, 3];
    display_array(&arr);

    let arr: [i32;2] = [1,2];
    display_array(&arr);
}

fn display_array<T: std::fmt::Debug>(arr: &[T]) {
    println!("{:?}", arr);
}

fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a + b
}
