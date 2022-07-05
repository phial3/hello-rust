fn main() {
    use std::collections::HashMap;

    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    let teams_map: HashMap<_,_> = teams_list.into_iter().collect();

    println!("{:?}",teams_map);

    // let name = String::from("Sunface");
    // let age = 18;
    //
    // let mut handsome_boys = HashMap::new();
    // handsome_boys.insert(name, age);
    //
    // println!("因为过于无耻，{}已经被从帅气男孩名单中除名", name);
    // println!("还有，他的真实年龄远远不止{}岁", age);
    //
    // let name = String::from("Sunface");
    // let age = 18;
    //
    // let mut handsome_boys = HashMap::new();
    // handsome_boys.insert(&name, age);
    //
    // std::mem::drop(name);
    // println!("因为过于无耻，{:?}已经被除名", handsome_boys);
    // println!("还有，他的真实年龄远远不止{}岁", age);

    let mut scores = HashMap::new();

    scores.insert("Blue", 10);

    // 覆盖已有的值
    let old = scores.insert("Blue", 20);
    assert_eq!(old, Some(10));

    // 查询新插入的值
    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(5);
    assert_eq!(*v, 5); // 不存在，插入5

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(50);
    assert_eq!(*v, 5); // 已经存在，因此50没有插入

    let mut contacts = HashMap::new();
    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", 234);
}
