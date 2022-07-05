use std::collections::HashMap;

#[derive(Debug)]
struct Contact {
    phone: String,
    address: String,
}

fn main() {
    let mut phones: HashMap<&str, String> = HashMap::new();
    phones.insert("Daniel", "798-1364".into());
    phones.insert("Ashley", "645-7689".into());
    phones.insert("Katie", "435-8291".into());
    phones.insert("Robert", "956-1745".into());

    let mut addresses: HashMap<&str, String> = HashMap::new();
    addresses.insert("Daniel", "12 A Street".into());
    addresses.insert("Ashley", "12 B Street".into());
    addresses.insert("Katie", "12 C Street".into());
    addresses.insert("Robert", "12 D Street".into());

    // let contacts: HashMap<&str, Contact> = phones.keys().fold(HashMap::new(), |mut acc, value| {
    //     acc.entry(value).or_insert(Contact {
    //         phone: *phones.get(value).unwrap(),
    //         address: *addresses.get(value).unwrap(),
    //     });
    //     acc
    // });

    let contacts: HashMap<_, _> = phones
        .into_iter()
        .map(|(key, phone)| {
            (
                key,
                Contact {
                    phone,
                    address: addresses.remove(key).unwrap(),
                },
            )
        })
        .collect();

    println!("{:?}", contacts);
}
