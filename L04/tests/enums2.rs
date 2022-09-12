// I AM NOT DONE

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move { _x: i32, _y: i32 },
    Echo (String),
    ChangeColor(u8,u8,u8),
    Quit
}

impl Message {
    fn call(&self) {
        println!("text {:?}", &self);
    }
}

#[cfg(test)]
fn main() {
    let messages = [
        Message::Move { _x: 10, _y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
