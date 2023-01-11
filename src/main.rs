use exper_message_trait::{Message, ProcessMsg};
use std::sync::mpsc::channel;

mod sm_enum_messages;
use sm_enum_messages::{Messages, SmEnumMessages};

mod sm_individual_messages;
use sm_individual_messages::{Move, Quit, SmIndividualMessages, Write};

fn run_enum() {
    let (tx, rx) = channel::<Box<Message>>();

    // Create the state machine
    let mut mysm = SmEnumMessages::new(SmEnumMessages::state0);

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

    println!("mysm: {mysm:#?}");
}

fn run_individual() {
    let (tx, rx) = channel::<Box<Message>>();

    // Create the state machine
    let mut mysm = SmIndividualMessages::new(SmIndividualMessages::state0);

    // Send Write msg, receive it and then process it to mysm
    _ = tx.send(Box::new(Write(String::from("Hello, world!"))));
    let msg = rx.recv().unwrap();
    mysm.process_msg(msg);

    // Send Move msg, receive it and then process it to mysm
    _ = tx.send(Box::new(Move { x: 1, y: 2 }));
    let msg = rx.recv().unwrap();
    mysm.process_msg(msg);

    // Send Anoter Write msg, receive it and then process it to mysm
    _ = tx.send(Box::new(Write(String::from("Yo, dude!"))));
    let msg = rx.recv().unwrap();
    mysm.process_msg(msg);

    // Send Anoter Quit msg, receive it and then process it to mysm
    _ = tx.send(Box::new(Quit));
    let msg = rx.recv().unwrap();
    mysm.process_msg(msg);

    println!("mysm: {mysm:#?}");
}

fn main() {
    run_enum();
    run_individual();
}
