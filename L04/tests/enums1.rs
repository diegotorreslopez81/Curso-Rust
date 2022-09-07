// I AM NOT DONE

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit(String),
    Echo(bool),
    Move { x: i32, y: i32 },
    ChangeColor(i32,i32,i32) 
}

#[cfg(test)]
mod tests {
    use crate::Message;

    #[test]
    fn call_enum() {
        
        println!("{:?}", Message::Quit(String::from("quit")));
        println!("{:?}", Message::Echo(true));
        println!("{:?}", Message::Move{ x: 2, y: 3 });
        println!("{:?}", Message::ChangeColor(231,45,123));    
    }

}

