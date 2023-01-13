use std::any::Any;

// Define your structs
struct Message1 {
    data: i32,
}

struct Message2 {
    data: String,
}

fn handle_message(message: Box<dyn Any>) {
    // Use the downcast_ref() method to extract the struct from the Box<dyn Any>
    if let Some(message1) = message.downcast_ref::<Message1>() {
        println!("Received message1 with data: {}", message1.data);
    } else if let Some(message2) = message.downcast_ref::<Message2>() {
        println!("Received message2 with data: {}", message2.data);
    } else {
        println!("Received unknown message");
    }
}

fn main() {
    let message1 = Message1 { data: 42 };
    let message2 = Message2 {
        data: "Hello".to_string(),
    };

    // Use the boxed() method to create a Box<dyn Any> object
    let message1_boxed = Box::new(message1);
    let message2_boxed = Box::new(message2);

    handle_message(message1_boxed); // Output: Received message1 with data: 42
    handle_message(message2_boxed); // Output: Received message2 with data: Hello
}
