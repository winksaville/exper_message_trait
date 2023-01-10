use std::any::Any;

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

impl Message {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    let binding = Message::Write(String::from("Hello, world!"));
    let messages: Vec<&dyn Any> = vec![
        Message::Quit.as_any(),
        Message::Move { x: 10, y: 20 }.as_any(),
        binding.as_any(),
    ];

    for message_any in messages {
        match message_any.downcast_ref::<Message>() {
            Some(Message::Quit) => println!("Received Quit message"),
            Some(Message::Move { x, y }) => println!("Received Move message: ({}, {})", x, y),
            Some(Message::Write(s)) => println!("Received Write message: {}", s),
            None => println!("Received unknown message"),
        }
    }
}
