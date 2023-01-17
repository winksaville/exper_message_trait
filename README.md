# Experiment with message trait

Based on a ChatGPT conversation, the original chat is gone, but
see [examples/chatgpt_code.md](/examples/chatgpt_code.md) for
a little more information.

The project show four styles for sending messages to "actors". Here these
actors here are implemented as simple state machines and are sent
messages which constructed using Rust `String`, `enum` or `struct` objects.

The four styles are:
 * `sm_string_msgs.rs` where each message is a `String`.
 * `sm_enum_msgs.rs` where each message is an `EnumMsgs` defined in `lib.rs`.
 * `sm_enum_msgs_any.rs` where each msg is a `Box<MsgAny>`, but where `EnumMsgs` are passed.
 * `sm_separate_msgs.rs` where each msg is a `Box<MsgAny>`, but where  different `struct`s are passed.

I should note that when using `MsgAny`, type alias for `dyn Any`, that any type
of message can be passed including `String` so it is the most general. The
question I'm trying to resolve is what is the "best" type of message.

I have mostly been using `EnumMsgs` to date, but it's problematic from a system
point of view. What you'd like to be able to do is create an actor define what
messages it supports. When using `EnumMsgs` if an actor is going to be included
in a system then it's messages must be added to the `EnumMsgs` and every actor
and the system needs to be recompiled. Using `MsgAny` is a path to a solution,
although currently Rust TypeId's are not universal across binaries, but I believe
that can be [resolved](https://www.google.com/search?q=rust+universal+typeid).
I'll be attempting to create a `universal TypeId` soon.

In the near term I'll be using `MsgAny`/`dyn Any` as this set of experiments
have convinced me its the "best" choice for now.

Here are the relavent files:
 * [lib.rs](/src/lib.rs)
 * [sm_string_msgs.rs](/src/sm_string_msgs.rs), msgs passed as `String`.
 * [sm_enum_msgs.rs](/src/sm_enum_msgs.rs), msgs passed `as-is`
 * [sm_enum_msgs_any.rs](/src/sm_enum_msgs_any.rs), enum msgs passed via `dyn Any`.
 * [sm_separate_msgs_any.rs](/src/sm_separate_msgs_any.rs), individual `structs`'s passed via `dyn Any`.
 * [main.rs](/src/main.rs), a main which runs all three types.
 * [crit.rs](/benches/crit.rs), benchmarks.

## Examples

  * [examples](/examples/README.md)

## Run:

```
$ cargo run
   Compiling exper_message_trait v0.10.0 (/home/wink/prgs/rust/myrepos/exper_message_trait)
    Finished dev [unoptimized + debuginfo] target(s) in 0.44s
     Running `target/debug/exper_message_trait`
mysm: SmStringMsgs {
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
mysm: SmEnumMsgs {
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
mysm: SmEnumMsgsAny {
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
mysm: SmSeparateMsgsAny {
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
```

## Benchmarks:

There are four benchmarks `quit`, `write`, `move` and `all`:

 * `quit`: Processes a message with just an action.
 * `write`: Processes a message with an action and a string parameter.
 * `move`: Processes a message with an action and two integer parameters.
 * `all`: Processes all 3 messages.

In each of those benchmarks there are 4 inner benchmarks:

 * `separate_msgs_any`: Each message is a separate `struct` and passed as a `Box<MsgAny>`.
 * `enum_any`: Each message is a member of an `enum EnumMsg` and passed as a `Box<MsgAny>`.
 * `enum_msgs`: Each message is a member of an `enum EnumMsg` and passed as a `Box<EnumMsg>`.
 * `string_msg`: Each message is a `String` and passed as a `String`.

> Note: In Rust a Box:<Xxx> means `Xxx` is allocated on the heap and in Rust `String` is allocated on the heap.

Look in [crit.rs](/benches/crit.rs) for the actual benchmarks and
below is the `all` benchmark:
```
fn all_bench(c: &mut Criterion) {
    //println!("all:+");

    let plot_config = PlotConfiguration::default();

    let mut group = c.benchmark_group("all");
    group.plot_config(plot_config);

    group.bench_function("separate_msgs_any", |b| {
        let mut sm = SmSeparateMsgsAny::new(SmSeparateMsgsAny::state0);

        let mut x = Wrapping(1i32);
        let mut y = Wrapping(2i32);
        b.iter(|| {
            x += 1;
            y += 1;
            let mm = Move { x, y };
            let bmm = Box::new(mm);
            sm.process_msg_any(bmm);

            let mw = Write("Hi".to_owned());
            let bmw = Box::new(mw);
            sm.process_msg_any(bmw);

            let mq = Quit;
            let bmq = Box::new(mq);
            sm.process_msg_any(bmq);
        });
    });

    group.bench_function("enum_any", |b| {
        let mut sm = SmEnumMsgsAny::new(SmEnumMsgsAny::state0);

        let mut x = Wrapping(1i32);
        let mut y = Wrapping(2i32);
        b.iter(|| {
            x += 1;
            y += 1;
            let mm = EnumMsgs::Move { x, y };
            let bmm = Box::new(mm);
            sm.process_msg_any(bmm);

            let mw = EnumMsgs::Write("Hi".to_owned());
            let bmw = Box::new(mw);
            sm.process_msg_any(bmw);

            let mq = EnumMsgs::Quit;
            let bmq = Box::new(mq);
            sm.process_msg_any(bmq);
        });
    });

    group.bench_function("enum_msgs", |b| {
        let mut sm = SmEnumMsgs::new(SmEnumMsgs::state0);

        let mut x = Wrapping(1i32);
        let mut y = Wrapping(2i32);
        b.iter(|| {
            x += 1;
            y += 1;
            let mm = EnumMsgs::Move { x, y };
            let bmm = Box::new(mm);
            sm.process_msg(bmm);

            let mw = EnumMsgs::Write("Hi".to_owned());
            let bmw = Box::new(mw);
            sm.process_msg(bmw);

            let mq = EnumMsgs::Quit;
            let bmq = Box::new(mq);
            sm.process_msg(bmq);
        });
    });

    group.bench_function("string_msg", |b| {
        let mut sm = SmStringMsgs::new(SmStringMsgs::state0);

        let mut x = Wrapping(1i32);
        let mut y = Wrapping(2i32);
        b.iter(|| {
            x += 1;
            y += 1;
            let mm = format!("Move x {} y {}", x, y);
            sm.process_string_msg(mm);

            let mw = String::from("Write Hello, world!");
            sm.process_string_msg(mw);

            let mq = String::from("Quit");
            sm.process_string_msg(mq);
        });
    });

    //println!("all:-");
}
```

Note: See issue #1

Below is a simple table of the times and [here are the graphs for the benchmarks](https://htmlpreview.github.io/?https://github.com/winksaville/exper_message_trait/blob/main/benches/results/criterion/reports/index.html).

```
$ ./benchmarks.sh 
   Compiling autocfg v1.1.0
   Compiling proc-macro2 v1.0.49
   ...
   Compiling tinytemplate v1.2.1
   Compiling criterion v0.4.0
    Finished bench [optimized] target(s) in 10.69s

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    Finished bench [optimized] target(s) in 0.03s
all/separate_msgs_any   time:   [42.201 ns 42.329 ns 42.468 ns]                                   
all/enum_any            time:   [48.673 ns 48.830 ns 48.992 ns]                          
all/enum_msgs           time:   [39.604 ns 39.638 ns 39.675 ns]                           
all/string_msg          time:   [305.58 ns 306.95 ns 308.04 ns]                           

quit/separate_msgs_any  time:   [4.4949 ns 4.4977 ns 4.5014 ns]                                    
quit/enum_any           time:   [11.544 ns 11.553 ns 11.565 ns]                           
quit/enum_msgs          time:   [9.7169 ns 9.8287 ns 9.9658 ns]                            
quit/string_msg         time:   [15.140 ns 15.150 ns 15.162 ns]                             

write/separate_msgs_any time:   [20.573 ns 20.599 ns 20.635 ns]                                     
write/enum_any          time:   [21.508 ns 21.562 ns 21.614 ns]                            
write/enum_msgs         time:   [19.698 ns 19.736 ns 19.777 ns]                             
write/string_msg        time:   [30.433 ns 30.461 ns 30.493 ns]                              

move/separate_msgs_any  time:   [12.649 ns 12.660 ns 12.677 ns]                                    
move/enum_any           time:   [14.698 ns 14.795 ns 14.894 ns]                           
move/enum_msgs          time:   [12.067 ns 12.091 ns 12.126 ns]                            
move/string_msg         time:   [235.36 ns 235.92 ns 236.36 ns]                            


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
