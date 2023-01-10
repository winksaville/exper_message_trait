use exper_message_trait::Message;

#[derive(Debug)]
enum Messages {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn main() {
    let messages: Vec<Box<Message>> = vec![
        Box::new(Messages::Quit),
        Box::new(Messages::Move { x: 10, y: 20 }),
        Box::new(Messages::Write(String::from("Hello, world!"))),
    ];

    for message_any in messages {
        match message_any.downcast_ref::<Messages>() {
            Some(Messages::Quit) => println!("Received Quit message"),
            Some(Messages::Move { x, y }) => println!("Received Move message: ({}, {})", x, y),
            Some(Messages::Write(s)) => println!("Received Write message: {}", s),
            None => println!("Received unknown message"),
        }
    }
}
