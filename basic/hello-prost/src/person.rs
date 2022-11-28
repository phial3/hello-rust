use prost::Message;

mod pb;

use pb::Person;
use pb::PhoneNumber;
use pb::PhoneType;

fn main() {
    let phones = vec![PhoneNumber::new("12345678", PhoneType::Work)];
    let person = Person::new("Daniel", 1, "daniel@work.com", phones);
    let v1 = person.encode_to_vec();
    let v2 = person.encode_length_delimited_to_vec();

    println!("person = {person:?}");
    println!("v1: len = {}, value = {v1:?}", v1.len());
    println!("v2 = {v2:?}");

    let person1 = Person::decode(v1.as_ref()).unwrap();
    assert_eq!(person, person1);

    let person1_json = serde_json::to_string_pretty(&person1).unwrap();
    println!("person1_json = {person1_json}");
}