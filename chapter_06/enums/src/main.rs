#[derive(Debug)]
enum Message {
    Quit,
    Move{ x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn info(&self) -> String {
        match self {
            Message::Quit => String::from("quit message"),
            Message::Move{x, y} => format!("move message x={}, y={}", x, y),
            _ => String::from("write or changecolor message")
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i+1),
        None => None
    }
}

fn main() {
    let messages = [
        Message::Quit,
        Message::Move{x: 2, y: 3},
        Message::Write(String::from("foo")),
        Message::ChangeColor(5, 6, 7),
    ];
    for message in messages.iter() {
        println!("message = {}", message.info())
    }

    let five = Some(5);
    let six = plus_one(five);
    println!("six = {:#?}", six);

    let none = plus_one(None);
    println!("none = {:#?}", none);

    if let Some(6) = six {
        println!("yay!")
    }
}
