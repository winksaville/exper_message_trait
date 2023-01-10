# Experiment with message trait

I [asked ChatGPT](https://chat.openai.com/chat/43127a48-08e0-4503-86eb-cb309ba89214)
to help me using traits as messages and this is what we came up with:

```
$ cat -n src/main.rs
     1	use std::any::Any;
     2	
     3	#[derive(Debug)]
     4	enum Message {
     5	    Quit,
     6	    Move { x: i32, y: i32 },
     7	    Write(String),
     8	}
     9	
    10	impl Message {
    11	    fn as_any(&self) -> &dyn Any {
    12	        self
    13	    }
    14	}
    15	
    16	fn main() {
    17	    let binding = Message::Write(String::from("Hello, world!"));
    18	    let messages: Vec<&dyn Any> = vec![
    19	        Message::Quit.as_any(),
    20	        Message::Move { x: 10, y: 20 }.as_any(),
    21	        binding.as_any(),
    22	    ];
    23	
    24	    for message_any in messages {
    25	        match message_any.downcast_ref::<Message>() {
    26	            Some(Message::Quit) => println!("Received Quit message"),
    27	            Some(Message::Move { x, y }) => println!("Received Move message: ({}, {})", x, y),
    28	            Some(Message::Write(s)) => println!("Received Write message: {}", s),
    29	            None => println!("Received unknown message"),
    30	        }
    31	    }
    32	}
```

## Run:

```
$ cargo run
   Compiling exper_message_trait v0.1.0 (/home/wink/prgs/rust/myrepos/exper_message_trait)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/exper_message_trait`
Received Quit message
Received Move message: (10, 20)
Received Write message: Hello, world!
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
