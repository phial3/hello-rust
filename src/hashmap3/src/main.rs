use std::collections::HashMap;

fn do_it(map: &mut HashMap<String, String>) {
    // for (key, value) in &*map {
    //     println!("{} / {}", key, value);
    //     //map.remove(key);
    // }
    // map.clear();

    // map.retain(|key, value| {
    //     println!("{} / {}", key, value);
    //
    //     !key.starts_with("a")
    // })

    // for (key, value) in map.into_iter() {
    //     println!("{} / {}", key, value);
    //     map.remove(key);
    // }

    // let mut to_remove = Vec::new();
    // for (key, value) in &*map {
    //     if key.starts_with("A") {
    //         to_remove.push(key.to_owned());
    //     }
    // }
    // for key in to_remove.iter() {
    //     map.remove(key);
    // }

    *map = map.into_iter().filter_map(|(key, value)| {
        if key.starts_with("A") {
            None
        } else {
            Some((key.to_owned(), value.to_owned()))
        }
    }).collect();
}

fn main() {}
