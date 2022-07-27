enum Direction {
    East,
    West,
    North,
    South,
}

enum IpAddr {
    Ipv4,
    Ipv6
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
        _ => println!("West"),
    };

    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };

    println!("{}", ip_str);

    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1,2),
        Action::ChangeColorRGB(255,255,0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            },
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            },
            Action::ChangeColorRGB(r, g, _) => {
                println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                         r, g,
                );
            }
        }
    }

    let age = Some(30);
    println!("在匹配前，age是{:?}",age);
    if let Some(age) = age {
        println!("匹配出来的age是{}",age);
    }

    println!("在匹配后，age是{:?}",age);

    let age = Some(30);
    println!("在匹配前，age是{:?}",age);
    match age {
        Some(age) =>  println!("匹配出来的age是{}",age),
        _ => ()
    }
    println!("在匹配后，age是{:?}",age);

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
}

enum Message {
    Hello { id: i32 },
}
