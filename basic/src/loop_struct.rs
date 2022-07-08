fn main() {
    let s = Student {
        name: String::from("a"),
        age: 16,
    };

    let s1 = Student {
        name: String::from("a"),
        age: 16,
    };

    let mut students = Vec::new();
    students.push(s);
    students.push(s1);

    for stu in students {
        stu.hello(stu.name.clone());
        stu.say(stu.age);
    }
}

pub struct Student {
    name: String,
    age: u32,
}

impl Student {
    pub fn hello(&self, name: String) {
        println!("Hello, {}", name);
    }

    pub fn say(&self, age: u32) {
        println!("Say, {}", age);
    }
}
