fn main() {
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
