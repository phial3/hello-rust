/// https://blog.csdn.net/u011528645/article/details/123117829
fn main() {
    // println!("Hello, world!");

    // let mut items:Vec<&str> = vec!["a", "b", "c", "c", "d", "e"];
    // println!("before items is {:?}", items);
    // for (index, item) in items.iter().enumerate() {
    //     if *item == "c" {
    //         items.remove(index);
    //     }
    // }
    // println!("after items is {:?}", items);

    // let mut items: Vec<&str> = vec!["a", "b", "c", "c", "d", "e"];
    // println!("before items is {:?}", items);
    // let mut remove_indexs: Vec<usize> = Vec::new();
    // for (index, item) in items.iter().enumerate() {
    //     if *item == "c" {
    //         remove_indexs.push(index);
    //     }
    // }
    // println!("remove indexs is {:?}", remove_indexs);
    // for i in remove_indexs {
    //     items.remove(i);
    // }
    // println!("then items is {:?}", items);

    let mut items: Vec<&str> = vec!["a", "b", "c", "c", "d", "e"];
    println!("before items is {:?}", items);
    items.retain(|item| if *item == "c" { false } else { true });
    println!("then items is {:?}", items);
}
