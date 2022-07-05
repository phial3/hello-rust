fn main() {
    //println!("Hello, world!");
    // let my_name = "Pascal";
    // greet(my_name);

    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear(); // error!
    // println!("the first word is: {}", word);

    // let a = [1, 2, 3, 4, 5];
    // let slice = &a[1..3];
    // assert_eq!(slice, &[2, 3]);
    //
    // let s: &str = "Hello, world!";

    let mut s = String::from("Hello ");
    s.push('r');
    println!("追加字符 push() -> {}", s);

    s.push_str("ust!");
    println!("追加字符串 push_str() -> {}", s);

    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(new_string_replace);

    let string_replace = "I like rust. Learning rust is my favorite!";
    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
    dbg!(new_string_replacen);

    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);

    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);

    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字
    string_remove.remove(0);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    // string_remove.remove(3);
    dbg!(string_remove);

    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);

    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);

    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str
    let result = string_append + &string_rust;
    let mut result = result + "!";
    result += "!!!";

    println!("连接字符串 + -> {}", result);
}

// fn first_word(s: &String) -> &str {
//     &s[..1]
// }

// fn greet(name: String) {
//     println!("Hello, {}!", name);
// }