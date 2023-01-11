# Experiment with message trait

Based on a [ChatGPT conversation](https://chat.openai.com/chat/43127a48-08e0-4503-86eb-cb309ba89214).

lib.rs contains `trait ProcessMsg` and type alias `Message` for `dyn Any` for
convience it exports example state machines; `sm_enum_messages` and `sm_individual_messages`.
These show two different ways of organizing messages, either bundle them in to an
`enum` or using separate `struct`'s and you can mix and match although using one
or the other will make the code more consistent. In the future I envision using
macros which might make things simpler.

lib.rs:
```
$ cat -n src/lib.rs
     1	use std::any::Any;
     2	
     3	// Messages are things that implement trait std::any::Any
     4	// which is most anything
     5	pub type Message = dyn Any;
     6	
     7	// This type alias is generic and apparently can't be exported
     8	// but Message can, oh well.
     9	//pub type SmProcessMsgFn<SM> = fn(&mut SM, Box<Message>);
    10	
    11	// Dispatch a message
    12	pub trait ProcessMsg {
    13	    fn process_msg(&mut self, msg: Box<Message>);
    14	}
    15	
    16	pub mod sm_enum_messages;
    17	pub mod sm_individual_messages;
```

sm_enum_messages.rs:
```
$ cat -n src/sm_enum_messages.rs 
     1	use crate::{Message, ProcessMsg};
     2	use std::fmt::{self, Debug};
     3	
     4	// Why do I have to declare a type alias here, I'd like to `use` it?
     5	//    use crate::SmProcessMsgFn;
     6	pub type SmProcessMsgFn<SM> = fn(&mut SM, Box<Message>);
     7	
     8	#[derive(Debug, Clone)]
     9	#[allow(unused)]
    10	pub enum Messages {
    11	    Quit,
    12	    Move { x: i32, y: i32 },
    13	    Write(String),
    14	}
    15	
    16	pub struct SmEnumMessages {
    17	    current_state: SmProcessMsgFn<Self>,
    18	    pub state0_counter: usize,
    19	    pub state0_quit_counter: usize,
    20	    pub state0_move_counter: usize,
    21	    pub state0_move_xy_counter: usize,
    22	    pub state0_write_counter: usize,
    23	    pub state0_write_sum_len_s: usize,
    24	    pub state0_none_counter: usize,
    25	
    26	    pub state1_counter: usize,
    27	    pub state1_quit_counter: usize,
    28	    pub state1_move_counter: usize,
    29	    pub state1_move_xy_counter: usize,
    30	    pub state1_write_counter: usize,
    31	    pub state1_write_sum_len_s: usize,
    32	    pub state1_none_counter: usize,
    33	}
    34	
    35	impl Debug for SmEnumMessages {
    36	    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    37	        f.debug_struct("SmEnumMessages")
    38	            //.field("current_state", &self.current_state)
    39	            .field("state0_counter", &self.state0_counter)
    40	            .field("state0_quit_counter", &self.state0_quit_counter)
    41	            .field("state0_move_counter", &self.state0_move_counter)
    42	            .field("state0_move_xy_counter", &self.state0_move_xy_counter)
    43	            .field("state0_write_counter", &self.state0_write_counter)
    44	            .field(
    45	                "state0_write_sum_len_s_counter",
    46	                &self.state0_write_sum_len_s,
    47	            )
    48	            .field("state0_none_counter", &self.state0_none_counter)
    49	            .field("state1_counter", &self.state1_counter)
    50	            .field("state1_quit_counter", &self.state1_quit_counter)
    51	            .field("state1_move_counter", &self.state1_move_counter)
    52	            .field("state1_move_xy_counter", &self.state1_move_xy_counter)
    53	            .field("state1_write_counter", &self.state1_write_counter)
    54	            .field(
    55	                "state1_write_sum_len_s_counter",
    56	                &self.state1_write_sum_len_s,
    57	            )
    58	            .field("state1_none_counter", &self.state1_none_counter)
    59	            .finish()
    60	    }
    61	}
    62	
    63	#[allow(unused)]
    64	impl SmEnumMessages {
    65	    pub fn new(initial_state: SmProcessMsgFn<Self>) -> Self {
    66	        Self {
    67	            current_state: initial_state,
    68	            state0_counter: 0,
    69	            state0_quit_counter: 0,
    70	            state0_move_counter: 0,
    71	            state0_move_xy_counter: 0,
    72	            state0_write_counter: 0,
    73	            state0_write_sum_len_s: 0,
    74	            state0_none_counter: 0,
    75	
    76	            state1_counter: 0,
    77	            state1_quit_counter: 0,
    78	            state1_move_counter: 0,
    79	            state1_move_xy_counter: 0,
    80	            state1_write_counter: 0,
    81	            state1_write_sum_len_s: 0,
    82	            state1_none_counter: 0,
    83	        }
    84	    }
    85	
    86	    fn transition(&mut self, dest: SmProcessMsgFn<Self>) {
    87	        self.current_state = dest;
    88	    }
    89	
    90	    pub fn state0(&mut self, msg: Box<Message>) {
    91	        self.state0_counter += 1;
    92	        match msg.downcast_ref::<Messages>() {
    93	            Some(Messages::Quit) => self.state0_quit_counter += 1,
    94	            Some(Messages::Move { x, y }) => {
    95	                self.state0_move_counter += 1;
    96	                self.state0_move_xy_counter += x.unsigned_abs() as usize + y.unsigned_abs() as usize;
    97	            }
    98	            Some(Messages::Write(s)) => {
    99	                self.state0_write_counter += 1;
   100	                self.state0_write_sum_len_s += s.len();
   101	            }
   102	            None => self.state0_none_counter += 1,
   103	        }
   104	
   105	        self.transition(SmEnumMessages::state1);
   106	    }
   107	
   108	    pub fn state1(&mut self, msg: Box<Message>) {
   109	        self.state1_counter += 1;
   110	        match msg.downcast_ref::<Messages>() {
   111	            Some(Messages::Quit) => self.state1_quit_counter += 1,
   112	            Some(Messages::Move { x, y }) => {
   113	                self.state1_move_counter += 1;
   114	                self.state1_move_xy_counter += x.unsigned_abs() as usize + y.unsigned_abs() as usize;
   115	            }
   116	            Some(Messages::Write(s)) => {
   117	                self.state1_write_counter += 1;
   118	                self.state1_write_sum_len_s += s.len();
   119	            }
   120	            None => self.state1_none_counter += 1,
   121	        }
   122	
   123	        self.transition(SmEnumMessages::state0);
   124	    }
   125	}
   126	
   127	impl ProcessMsg for SmEnumMessages {
   128	    fn process_msg(&mut self, msg: Box<Message>) {
   129	        (self.current_state)(self, msg);
   130	    }
   131	}
```

sm_individual_messages.rs:
```
$ cat -n src/sm_individual_messages.rs 
     1	use crate::{Message, ProcessMsg};
     2	use std::fmt::{self, Debug};
     3	
     4	// Why do I have to declare a type alias here, I'd like to `use` it?
     5	//    use crate::SmProcessMsgFn;
     6	pub type SmProcessMsgFn<SM> = fn(&mut SM, Box<Message>);
     7	
     8	#[derive(Debug, Clone)]
     9	pub struct Quit;
    10	
    11	#[derive(Debug, Clone)]
    12	pub struct Move {
    13	    pub x: i32,
    14	    pub y: i32,
    15	}
    16	
    17	#[derive(Debug, Clone)]
    18	pub struct Write(pub String);
    19	
    20	//#[derive(Debug)]
    21	pub struct SmIndividualMessages {
    22	    current_state: SmProcessMsgFn<Self>,
    23	    pub state0_counter: usize,
    24	    pub state0_quit_counter: usize,
    25	    pub state0_move_counter: usize,
    26	    pub state0_move_xy_counter: usize,
    27	    pub state0_write_counter: usize,
    28	    pub state0_write_sum_len_s: usize,
    29	    pub state0_none_counter: usize,
    30	
    31	    pub state1_counter: usize,
    32	    pub state1_quit_counter: usize,
    33	    pub state1_move_counter: usize,
    34	    pub state1_move_xy_counter: usize,
    35	    pub state1_write_counter: usize,
    36	    pub state1_write_sum_len_s: usize,
    37	    pub state1_none_counter: usize,
    38	}
    39	
    40	impl Debug for SmIndividualMessages {
    41	    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    42	        f.debug_struct("SmIndividualMessages")
    43	            //.field("current_state", &self.current_state)
    44	            .field("state0_counter", &self.state0_counter)
    45	            .field("state0_quit_counter", &self.state0_quit_counter)
    46	            .field("state0_move_counter", &self.state0_move_counter)
    47	            .field("state0_move_xy_counter", &self.state0_move_xy_counter)
    48	            .field("state0_write_counter", &self.state0_write_counter)
    49	            .field(
    50	                "state0_write_sum_len_s_counter",
    51	                &self.state0_write_sum_len_s,
    52	            )
    53	            .field("state0_none_counter", &self.state0_none_counter)
    54	            .field("state1_counter", &self.state1_counter)
    55	            .field("state1_quit_counter", &self.state1_quit_counter)
    56	            .field("state1_move_counter", &self.state1_move_counter)
    57	            .field("state1_move_xy_counter", &self.state1_move_xy_counter)
    58	            .field("state1_write_counter", &self.state1_write_counter)
    59	            .field(
    60	                "state1_write_sum_len_s_counter",
    61	                &self.state1_write_sum_len_s,
    62	            )
    63	            .field("state1_none_counter", &self.state1_none_counter)
    64	            .finish()
    65	    }
    66	}
    67	
    68	#[allow(unused)]
    69	impl SmIndividualMessages {
    70	    pub fn new(initial_state: SmProcessMsgFn<Self>) -> Self {
    71	        Self {
    72	            current_state: initial_state,
    73	            state0_counter: 0,
    74	            state0_quit_counter: 0,
    75	            state0_move_counter: 0,
    76	            state0_move_xy_counter: 0,
    77	            state0_write_counter: 0,
    78	            state0_write_sum_len_s: 0,
    79	            state0_none_counter: 0,
    80	
    81	            state1_counter: 0,
    82	            state1_quit_counter: 0,
    83	            state1_move_counter: 0,
    84	            state1_move_xy_counter: 0,
    85	            state1_write_counter: 0,
    86	            state1_write_sum_len_s: 0,
    87	            state1_none_counter: 0,
    88	        }
    89	    }
    90	
    91	    fn transition(&mut self, dest: SmProcessMsgFn<Self>) {
    92	        self.current_state = dest;
    93	    }
    94	
    95	    pub fn state0(&mut self, msg: Box<Message>) {
    96	        self.state0_counter += 1;
    97	        if msg.downcast_ref::<Quit>().is_some() {
    98	            self.state0_quit_counter += 1;
    99	        } else if let Some(mm) = msg.downcast_ref::<Move>() {
   100	            self.state0_move_counter += 1;
   101	            self.state0_move_xy_counter += mm.x.unsigned_abs() as usize + mm.y.unsigned_abs() as usize;
   102	        } else if let Some(mw) = msg.downcast_ref::<Write>() {
   103	            self.state0_write_counter += 1;
   104	            self.state0_write_sum_len_s += mw.0.len();
   105	        } else {
   106	            self.state0_none_counter += 1;
   107	        }
   108	
   109	        self.transition(SmIndividualMessages::state1);
   110	    }
   111	
   112	    pub fn state1(&mut self, msg: Box<Message>) {
   113	        self.state1_counter += 1;
   114	        if msg.downcast_ref::<Quit>().is_some() {
   115	            self.state1_quit_counter += 1;
   116	        } else if let Some(mm) = msg.downcast_ref::<Move>() {
   117	            self.state1_move_counter += 1;
   118	            self.state1_move_xy_counter += mm.x.unsigned_abs() as usize + mm.y.unsigned_abs() as usize;
   119	        } else if let Some(mw) = msg.downcast_ref::<Write>() {
   120	            self.state1_write_counter += 1;
   121	            self.state1_write_sum_len_s += mw.0.len();
   122	        } else {
   123	            self.state1_none_counter += 1;
   124	        }
   125	
   126	        self.transition(SmIndividualMessages::state0);
   127	    }
   128	}
   129	
   130	impl ProcessMsg for SmIndividualMessages {
   131	    fn process_msg(&mut self, msg: Box<Message>) {
   132	        (self.current_state)(self, msg);
   133	    }
   134	}
```

main.rs:
```
$ cat -n src/main.rs
     1	use exper_message_trait::{Message, ProcessMsg};
     2	use std::sync::mpsc::channel;
     3	
     4	mod sm_enum_messages;
     5	use sm_enum_messages::{Messages, SmEnumMessages};
     6	
     7	mod sm_individual_messages;
     8	use sm_individual_messages::{Move, Quit, SmIndividualMessages, Write};
     9	
    10	fn run_enum() {
    11	    let (tx, rx) = channel::<Box<Message>>();
    12	
    13	    // Create the state machine
    14	    let mut mysm = SmEnumMessages::new(SmEnumMessages::state0);
    15	
    16	    // Send Write msg, receive it and then process it to mysm
    17	    _ = tx.send(Box::new(Messages::Write(String::from("Hello, world!"))));
    18	    let msg = rx.recv().unwrap();
    19	    mysm.process_msg(msg);
    20	
    21	    // Send Move msg, receive it and then process it to mysm
    22	    _ = tx.send(Box::new(Messages::Move { x: 1, y: 2 }));
    23	    let msg = rx.recv().unwrap();
    24	    mysm.process_msg(msg);
    25	
    26	    // Send Anoter Write msg, receive it and then process it to mysm
    27	    _ = tx.send(Box::new(Messages::Write(String::from("Yo, dude!"))));
    28	    let msg = rx.recv().unwrap();
    29	    mysm.process_msg(msg);
    30	
    31	    // Send Anoter Quit msg, receive it and then process it to mysm
    32	    _ = tx.send(Box::new(Messages::Quit));
    33	    let msg = rx.recv().unwrap();
    34	    mysm.process_msg(msg);
    35	
    36	    println!("mysm: {mysm:#?}");
    37	}
    38	
    39	fn run_individual() {
    40	    let (tx, rx) = channel::<Box<Message>>();
    41	
    42	    // Create the state machine
    43	    let mut mysm = SmIndividualMessages::new(SmIndividualMessages::state0);
    44	
    45	    // Send Write msg, receive it and then process it to mysm
    46	    _ = tx.send(Box::new(Write(String::from("Hello, world!"))));
    47	    let msg = rx.recv().unwrap();
    48	    mysm.process_msg(msg);
    49	
    50	    // Send Move msg, receive it and then process it to mysm
    51	    _ = tx.send(Box::new(Move { x: 1, y: 2 }));
    52	    let msg = rx.recv().unwrap();
    53	    mysm.process_msg(msg);
    54	
    55	    // Send Anoter Write msg, receive it and then process it to mysm
    56	    _ = tx.send(Box::new(Write(String::from("Yo, dude!"))));
    57	    let msg = rx.recv().unwrap();
    58	    mysm.process_msg(msg);
    59	
    60	    // Send Anoter Quit msg, receive it and then process it to mysm
    61	    _ = tx.send(Box::new(Quit));
    62	    let msg = rx.recv().unwrap();
    63	    mysm.process_msg(msg);
    64	
    65	    println!("mysm: {mysm:#?}");
    66	}
    67	
    68	fn main() {
    69	    run_enum();
    70	    run_individual();
    71	}
```

## Run:

```
$ cargo run
   Compiling exper_message_trait v0.4.0 (/home/wink/prgs/rust/myrepos/exper_message_trait)
    Finished dev [unoptimized + debuginfo] target(s) in 0.46s
     Running `target/debug/exper_message_trait`
mysm: SmEnumMessages {
    state0_counter: 2,
    state0_quit_counter: 0,
    state0_move_counter: 0,
    state0_move_xy_counter: 0,
    state0_write_counter: 2,
    state0_write_sum_len_s_counter: 22,
    state0_none_counter: 0,
    state1_counter: 2,
    state1_quit_counter: 1,
    state1_move_counter: 1,
    state1_move_xy_counter: 3,
    state1_write_counter: 0,
    state1_write_sum_len_s_counter: 0,
    state1_none_counter: 0,
}
mysm: SmIndividualMessages {
    state0_counter: 2,
    state0_quit_counter: 0,
    state0_move_counter: 0,
    state0_move_xy_counter: 0,
    state0_write_counter: 2,
    state0_write_sum_len_s_counter: 22,
    state0_none_counter: 0,
    state1_counter: 2,
    state1_quit_counter: 1,
    state1_move_counter: 1,
    state1_move_xy_counter: 3,
    state1_write_counter: 0,
    state1_write_sum_len_s_counter: 0,

```

## Benchmarks:

Currently it looks like using individual messages is a little faster than
using an enum, but I wouldn't bet the house on that.

```
$ taskset -c 0 cargo criterion
   Compiling exper_message_trait v0.4.0 (/home/wink/prgs/rust/myrepos/exper_message_trait)
    Finished bench [optimized] target(s) in 4.04s
sm_enum_messages/sm_enum_messages                                                                            
                        time:   [62.926 ns 62.981 ns 63.038 ns]
                        change: [-0.4193% -0.2067% -0.0055%] (p = 0.05 < 0.05)
                        Change within noise threshold.

sm_individual_messages/sm_individual_messages                                                                            
                        time:   [50.269 ns 50.315 ns 50.361 ns]
                        change: [-1.2277% -0.9052% -0.5832%] (p = 0.00 < 0.05)
                        Change within noise threshold.


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
