use exper_message_trait::{Message, ProcessMsg};
use std::sync::mpsc::channel;

#[derive(Debug)]
enum Messages {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

pub type SmProcessMsgFn<SM> = fn(&mut SM, Box<Message>);

pub struct MySm {
    current_state: SmProcessMsgFn<Self>,
}

impl MySm {
    pub fn new(initial_state: SmProcessMsgFn<Self>) -> Self {
        Self {
            current_state: initial_state,
        }
    }

    fn transition(&mut self, dest: SmProcessMsgFn<Self>) {
        self.current_state = dest;
    }

    pub fn state0(&mut self, msg: Box<Message>) {
        match msg.downcast_ref::<Messages>() {
            Some(Messages::Quit) => println!("state0: Received Quit message"),
            Some(Messages::Move { x, y }) => {
                println!("state0: Received Move message: ({x}, {y})");
            }
            Some(Messages::Write(s)) => println!("state0: Received Write message: {s}"),
            None => {
                println!("state0: Received unknown message: msg: {msg:?}");
            }
        }

        self.transition(MySm::state1);
    }

    pub fn state1(&mut self, msg: Box<Message>) {
        match msg.downcast_ref::<Messages>() {
            Some(Messages::Quit) => println!("state1: Received Quit message"),
            Some(Messages::Move { x, y }) => {
                println!("state1: Received Move message: ({x}, {y})");
            }
            Some(Messages::Write(s)) => println!("state1: Received Write message: {s}"),
            None => {
                println!("state1: Received unknown message: msg: {msg:?}");
            }
        }

        self.transition(MySm::state0);
    }
}

impl ProcessMsg for MySm {
    fn process_msg(&mut self, msg: Box<Message>) {
        (self.current_state)(self, msg);
    }
}

fn main() {
    let (tx, rx) = channel::<Box<Message>>();

    // Create the state machine
    let mut mysm = MySm::new(MySm::state0);

    // Send Write msg, receive it and then process it to mysm
    _ = tx.send(Box::new(Messages::Write(String::from("Hello, world!"))));
    let msg = rx.recv().unwrap();
    mysm.process_msg(msg);

    // Send Move msg, receive it and then process it to mysm
    _ = tx.send(Box::new(Messages::Move { x: 1, y: 2 }));
    let msg = rx.recv().unwrap();
    mysm.process_msg(msg);

    // Send Anoter Write msg, receive it and then process it to mysm
    _ = tx.send(Box::new(Messages::Write(String::from("Yo, dude!"))));
    let msg = rx.recv().unwrap();
    mysm.process_msg(msg);

    // Send Anoter Quit msg, receive it and then process it to mysm
    _ = tx.send(Box::new(Messages::Quit));
    let msg = rx.recv().unwrap();
    mysm.process_msg(msg);
}
