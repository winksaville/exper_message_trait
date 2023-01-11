# Experiment with message trait

Based on a [ChatGPT conversation](https://chat.openai.com/chat/43127a48-08e0-4503-86eb-cb309ba89214).

Add `trait ProcessMsg` which allows struct to be able to process messages. Then
I implemented a simple state machine with two states state0 and state1. These
states processes the messages and std::sync::channel is used to send and receive
messages.


lib.rs:

```
$ cat -n src/lib.rs
     1  use std::any::Any;
     2
     3  // Messages are things that implement trait std::any::Any
     4  // which is most anything
     5  pub type Message = dyn Any;
     6
     7  // Dispatch a message
     8  pub trait ProcessMsg {
     9      fn process_msg(&mut self, msg: Box<Message>);
    10  }
```

main.rs:
```
$ cat -n src/main.rs
     1  use exper_message_trait::{Message, ProcessMsg};
     2  use std::sync::mpsc::channel;
     3
     4  #[derive(Debug)]
     5  enum Messages {
     6      Quit,
     7      Move { x: i32, y: i32 },
     8      Write(String),
     9  }
    10
    11  pub type SmProcessMsgFn<SM> = fn(&mut SM, Box<Message>);
    12
    13  pub struct MySm {
    14      current_state: SmProcessMsgFn<Self>,
    15  }
    16
    17  impl MySm {
    18      pub fn new(initial_state: SmProcessMsgFn<Self>) -> Self {
    19          Self {
    20              current_state: initial_state,
    21          }
    22      }
    23
    24      fn transition(&mut self, dest: SmProcessMsgFn<Self>) {
    25          self.current_state = dest;
    26      }
    27
    28      pub fn state0(&mut self, msg: Box<Message>) {
    29          match msg.downcast_ref::<Messages>() {
    30              Some(Messages::Quit) => println!("state0: Received Quit message"),
    31              Some(Messages::Move { x, y }) => {
    32                  println!("state0: Received Move message: ({x}, {y})");
    33              }
    34              Some(Messages::Write(s)) => println!("state0: Received Write message: {s}"),
    35              None => {
    36                  println!("state0: Received unknown message: msg: {msg:?}");
    37              }
    38          }
    39
    40          self.transition(MySm::state1);
    41      }
    42
    43      pub fn state1(&mut self, msg: Box<Message>) {
    44          match msg.downcast_ref::<Messages>() {
    45              Some(Messages::Quit) => println!("state1: Received Quit message"),
    46              Some(Messages::Move { x, y }) => {
    47                  println!("state1: Received Move message: ({x}, {y})");
    48              }
    49              Some(Messages::Write(s)) => println!("state1: Received Write message: {s}"),
    50              None => {
    51                  println!("state1: Received unknown message: msg: {msg:?}");
    52              }
    53          }
    54
    55          self.transition(MySm::state0);
    56      }
    57  }
    58
    59  impl ProcessMsg for MySm {
    60      fn process_msg(&mut self, msg: Box<Message>) {
    61          (self.current_state)(self, msg);
    62      }
    63  }
    64
    65  fn main() {
    66      let (tx, rx) = channel::<Box<Message>>();
    67
    68      // Create the state machine
    69      let mut mysm = MySm::new(MySm::state0);
    70
    71      // Send Write msg, receive it and then process it to mysm
    72      _ = tx.send(Box::new(Messages::Write(String::from("Hello, world!"))));
    73      let msg = rx.recv().unwrap();
    74      mysm.process_msg(msg);
    75
    76      // Send Move msg, receive it and then process it to mysm
    77      _ = tx.send(Box::new(Messages::Move { x: 1, y: 2 }));
    78      let msg = rx.recv().unwrap();
    79      mysm.process_msg(msg);
    80
    81      // Send Anoter Write msg, receive it and then process it to mysm
    82      _ = tx.send(Box::new(Messages::Write(String::from("Yo, dude!"))));
    83      let msg = rx.recv().unwrap();
    84      mysm.process_msg(msg);
    85
    86      // Send Anoter Quit msg, receive it and then process it to mysm
    87      _ = tx.send(Box::new(Messages::Quit));
    88      let msg = rx.recv().unwrap();
    89      mysm.process_msg(msg);
    90  }
```

## Run:

```
$ cargo run
   Compiling cfg-if v1.0.0
   Compiling crossbeam-utils v0.8.14
   Compiling crossbeam-channel v0.5.6
   Compiling exper_message_trait v0.2.0 (/home/wink/prgs/rust/myrepos/exper_message_trait)
    Finished dev [unoptimized + debuginfo] target(s) in 0.88s
     Running `target/debug/exper_message_trait`
state0: Received Write message: Hello, world!
state1: Received Move message: (1, 2)
state0: Received Write message: Yo, dude!
state1: Received Quit message
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
