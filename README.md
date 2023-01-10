# Experiment with message trait

Based on a [ChatGPT conversation](https://chat.openai.com/chat/43127a48-08e0-4503-86eb-cb309ba89214).

Refactor and simplify. Create `lib.rs` with the `Message` type definition of
`Message = dyn Any`. And in `main.rs` removed the `as_any` trait function.

lib.rs:

```
$ cat -n src/lib.rs
     1	// Message as a `dyn Any` trait
     2	use std::any::Any;
     3	
     4	pub type Message = dyn Any;
```

main.rs:
```
$ cat -n src/main.rs
     1	use exper_message_trait::Message;
     2	
     3	#[derive(Debug)]
     4	enum Messages {
     5	    Quit,
     6	    Move { x: i32, y: i32 },
     7	    Write(String),
     8	}
     9	
    10	fn main() {
    11	    let messages: Vec<Box<Message>> = vec![
    12	        Box::new(Messages::Quit),
    13	        Box::new(Messages::Move { x: 10, y: 20 }),
    14	        Box::new(Messages::Write(String::from("Hello, world!"))),
    15	    ];
    16	
    17	    for message_any in messages {
    18	        match message_any.downcast_ref::<Messages>() {
    19	            Some(Messages::Quit) => println!("Received Quit message"),
    20	            Some(Messages::Move { x, y }) => println!("Received Move message: ({}, {})", x, y),
    21	            Some(Messages::Write(s)) => println!("Received Write message: {}", s),
    22	            None => println!("Received unknown message"),
    23	        }
    24	    }
    25	}
```

## Run:

```
$ cargo run
   Compiling exper_message_trait v0.1.0 (/home/wink/prgs/rust/myrepos/exper_message_trait)
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
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
