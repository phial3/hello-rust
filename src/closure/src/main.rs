
fn call(f: fn()) {
    f();
}

fn main() {
    let a = 1;

    let f = || println!("abc"); // anonymous function 函数指针
    let c = || println!("{}", &a);  // closure 闭包

    call(f);
    //call(c);

    // let f = |x: i32| -> i32 { x + 1 };
    // let f = |x: i32| x + 1;
    // let f = |x| x+1;
    // let f = |x| x;
    // let f = |x| x;
    // f('a');
    //f(i);

    let add_one = |x| x+1;
    // println!("{}", add_one(3800));

    let x: i8 = 126;
    println!("{}", add_one(x));

    let v1 = 100;
    let v2 = 100;
    let a = |x: i32| x;
    let b = |x: i32| x + v1;
    let c = |x: i32| x + v1 + v2;

    assert_eq!(size_of(&a), 0);
    assert_eq!(size_of(&b), 8);
    assert_eq!(size_of(&c), 16);
}

fn size_of<T>(_: &T) -> usize {
    std::mem::size_of::<T>()
}
