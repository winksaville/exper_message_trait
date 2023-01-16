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
 * [sm_string_split_msgs.rs](/src/sm_string_split_msgs.rs), msgs passed as `String`.
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
   Compiling exper_message_trait v0.9.0 (/home/wink/prgs/rust/myrepos/exper_message_trait)
    Finished dev [unoptimized + debuginfo] target(s) in 0.44s
     Running `target/debug/exper_message_trait`
mysm: SmStringSplitParseMsgs {
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

There are four benchmarks `quit`, `write`, `move` and `all`. Each of these test
a particular message sequence. `quit`, `write` and `move` are 
four benchmarks one each for the different type of messages, `separate_msgs_any`
`enum_any`, `enum_msgs` and `string_spit`.

Note: See issue #1

Below is a simple table of the times and [here are the graphs for the benchmarks](https://htmlpreview.github.io/?https://github.com/winksaville/exper_message_trait/blob/main/benches/results/criterion/reports/index.html).

```
$ ./benchmarks.sh 
   Compiling autocfg v1.1.0
   Compiling proc-macro2 v1.0.49
   ...
   Compiling tinytemplate v1.2.1
   Compiling criterion v0.4.0
    Finished bench [optimized] target(s) in 10.80s

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    Finished bench [optimized] target(s) in 0.03s
quit/separate_msgs_any  time:   [4.2910 ns 4.2953 ns 4.3002 ns]                                    
quit/enum_any           time:   [13.738 ns 13.796 ns 13.855 ns]                           
quit/enum_msgs          time:   [9.8410 ns 9.8945 ns 9.9505 ns]                            
quit/string_split       time:   [15.192 ns 15.205 ns 15.219 ns]                               

write/separate_msgs_any time:   [21.357 ns 21.486 ns 21.683 ns]                                     
write/enum_any          time:   [21.863 ns 21.944 ns 22.042 ns]                            
write/enum_msgs         time:   [20.623 ns 20.700 ns 20.780 ns]                             
write/string_split      time:   [31.760 ns 31.869 ns 31.983 ns]                                

move/separate_msgs_any  time:   [16.890 ns 16.932 ns 16.973 ns]                                    
move/enum_any           time:   [14.250 ns 14.305 ns 14.360 ns]                           
move/enum_msgs          time:   [12.225 ns 12.246 ns 12.266 ns]                            
move/string_split       time:   [215.72 ns 216.80 ns 217.67 ns]                              

all/separate_msgs_any   time:   [42.958 ns 43.076 ns 43.200 ns]                                   
all/enum_any            time:   [48.001 ns 48.162 ns 48.322 ns]                          
all/enum_msgs           time:   [39.745 ns 39.783 ns 39.829 ns]                           
all/string_split        time:   [275.95 ns 276.97 ns 277.81 ns]                             


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
