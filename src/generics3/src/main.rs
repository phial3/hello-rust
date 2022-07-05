fn main() {
    let container_i32 = Container::new(1);
    let container_str = Container::new(String::from("111"));

    container_i32.to_string();
    container_str.to_string();

    let stu = Student {
        name: "YSS".to_string(),
    };
    Student::who(&stu);
    Student::doing_at_night(&stu);
    Student::who_1(&stu);
    Student::doing_at_night_2(&stu);
}

struct Container<T: std::fmt::Display> {
    item: T,
}

impl<T: std::fmt::Display> Container<T> {
    fn new(item: T) -> Self {
        Container { item }
    }

    fn to_string(&self) -> String {
        self.item.to_string()
    }
}

trait Person {
    fn sleep() {
        println!("person need sleep");
    }

    fn get_name(&self) -> &str;
}

struct Student {
    name: String,
}

impl Person for Student {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl Student {
    fn who<T: Person>(anyone: &T) {
        println!("I'm {}", anyone.get_name());
    }

    fn who_1<T>(anyone: &T)
    where
        T: Person,
    {
        println!("I'm {}", anyone.get_name());
    }

    fn doing_at_night<T: Person>(_anyone: &T) {
        T::sleep();
    }

    fn doing_at_night_2<T>(_anyone: &T)
    where
        T: Person,
    {
        T::sleep();
    }
}
