use exper_message_trait::{MsgAny, ProcessMsgAny};
use std::{num::Wrapping, sync::mpsc::channel};

mod sm_enum_msgs_any;
use sm_enum_msgs_any::{Messages, SmEnumMsgsAny};

mod sm_individual_msgs_any;
use sm_individual_msgs_any::{Move, Quit, SmIndividualMsgsAny, Write};

mod sm_enum_msgs;
use sm_enum_msgs::{Msgs, ProcessMsg, SmEnumMsgs};

mod sm_string_msgs;
use sm_string_msgs::{ProcessStringMsg, SmStringMsgs};

fn run_string_msgs() {
    let (tx, rx) = channel::<String>();

    // Create the state machine
    let mut mysm = SmStringMsgs::new(SmStringMsgs::state0);

    // Send Write msg, receive it and then process it to mysm
    _ = tx.send(String::from("Write: Hello, world!"));
    let msg = rx.recv().unwrap();
    mysm.process_string_msg(msg);

    // Send Move msg, receive it and then process it to mysm
    let msg = format!("Move: {{x: {}, y: {}}}", 1, 2);
    _ = tx.send(msg);
    let msg = rx.recv().unwrap();
    mysm.process_string_msg(msg);

    // Send Anoter Write msg, receive it and then process it to mysm
    _ = tx.send(String::from("Write: Yo, dude!"));
    let msg = rx.recv().unwrap();
    mysm.process_string_msg(msg);

    // Send Anoter Quit msg, receive it and then process it to mysm
    _ = tx.send(String::from("Quit:"));
    let msg = rx.recv().unwrap();
    mysm.process_string_msg(msg);

    println!("mysm: {mysm:#?}");
}

fn run_enum_msgs() {
    let (tx, rx) = channel::<Box<Msgs>>();

    // Create the state machine
    let mut mysm = SmEnumMsgs::new(SmEnumMsgs::state0);

    // Send Write msg, receive it and then process it to mysm
    _ = tx.send(Box::new(Msgs::Write(String::from("Hello, world!"))));
    let msg = rx.recv().unwrap();
    mysm.process_msg(msg);

    // Send Move msg, receive it and then process it to mysm
    _ = tx.send(Box::new(Msgs::Move {
        x: Wrapping(1),
        y: Wrapping(2),
    }));
    let msg = rx.recv().unwrap();
    mysm.process_msg(msg);

    // Send Anoter Write msg, receive it and then process it to mysm
    _ = tx.send(Box::new(Msgs::Write(String::from("Yo, dude!"))));
    let msg = rx.recv().unwrap();
    mysm.process_msg(msg);

    // Send Anoter Quit msg, receive it and then process it to mysm
    _ = tx.send(Box::new(Msgs::Quit));
    let msg = rx.recv().unwrap();
    mysm.process_msg(msg);

    println!("mysm: {mysm:#?}");
}

fn run_enum_messages() {
    let (tx, rx) = channel::<Box<MsgAny>>();

    // Create the state machine
    let mut mysm = SmEnumMsgsAny::new(SmEnumMsgsAny::state0);

    // Send Write msg, receive it and then process it to mysm
    _ = tx.send(Box::new(Messages::Write(String::from("Hello, world!"))));
    let msg = rx.recv().unwrap();
    mysm.process_msg_any(msg);

    // Send Move msg, receive it and then process it to mysm
    _ = tx.send(Box::new(Messages::Move {
        x: Wrapping(1),
        y: Wrapping(2),
    }));
    let msg = rx.recv().unwrap();
    mysm.process_msg_any(msg);

    // Send Anoter Write msg, receive it and then process it to mysm
    _ = tx.send(Box::new(Messages::Write(String::from("Yo, dude!"))));
    let msg = rx.recv().unwrap();
    mysm.process_msg_any(msg);

    // Send Anoter Quit msg, receive it and then process it to mysm
    _ = tx.send(Box::new(Messages::Quit));
    let msg = rx.recv().unwrap();
    mysm.process_msg_any(msg);

    println!("mysm: {mysm:#?}");
}

fn run_individual_messages() {
    let (tx, rx) = channel::<Box<MsgAny>>();

    // Create the state machine
    let mut mysm = SmIndividualMsgsAny::new(SmIndividualMsgsAny::state0);

    // Send Write msg, receive it and then process it to mysm
    _ = tx.send(Box::new(Write(String::from("Hello, world!"))));
    let msg = rx.recv().unwrap();
    mysm.process_msg_any(msg);

    // Send Move msg, receive it and then process it to mysm
    _ = tx.send(Box::new(Move {
        x: Wrapping(1),
        y: Wrapping(2),
    }));
    let msg = rx.recv().unwrap();
    mysm.process_msg_any(msg);

    // Send Anoter Write msg, receive it and then process it to mysm
    _ = tx.send(Box::new(Write(String::from("Yo, dude!"))));
    let msg = rx.recv().unwrap();
    mysm.process_msg_any(msg);

    // Send Anoter Quit msg, receive it and then process it to mysm
    _ = tx.send(Box::new(Quit));
    let msg = rx.recv().unwrap();
    mysm.process_msg_any(msg);

    println!("mysm: {mysm:#?}");
}

fn main() {
    run_string_msgs();
    run_enum_msgs();
    run_enum_messages();
    run_individual_messages();
}
