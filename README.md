# Experiment with message trait

Based on a ChatGPT conversation, the original chat is gone, but
see [examples/chatgpt_code.md](/examples/chatgpt_code.md) for
a little more information.

The project show 3 styles for sending messages to "actors". Here these
actors here are implemented as simple state machines and are sent
messages which constructed using Rust `enum` or `struct` objects.

The three styles are:
 * Box<Message> where a `Message` is an `enum` or `struct` passed `as-is`.
 * Box<MsgAny> where a `MsgAny` is an `enum` passed as a `dyn Any`.
 * Box<MsgAny> where a `MsgAny` is individual `struct`'s but also passed as a `dyn Any`.

There is actually at least one more style, where you mix `enum`'s and individual `struct`'s.
At the moment I feel that is messey, although if `proc_macros` are eventually
used I think that will be perfectly reasonable thing to do as the messeyness
should be able to be hidden in the macro. Although, there looks to be a performance
hit when using `enum`'s and `dyn Any`.

Here are the relavent files:
 * [lib.rs](/src/lib.rs)
 * [sm_string_msgs.rs](/src/sm_string_msgs.rs), msgs passed as `String`.
 * [sm_enum_msgs.rs](/src/sm_enum_msgs.rs), msgs passed `as-is`
 * [sm_enum_msgs_any.rs](/src/sm_enum_msgs_any.rs), enum msgs passed via `dyn Any`.
 * [sm_individual_msgs_any.rs](/src/sm_individual_msgs_any.rs), individual `structs`'s passed via `dyn Any`.
 * [main.rs](/src/main.rs), a main which runs all three types.
 * [crit.rs](/benches/crit.rs), benchmarks.

## Examples

  * [examples](/examples/README.md)

## Run:

```
$ cargo run
   Compiling exper_message_trait v0.8.0 (/home/wink/prgs/rust/myrepos/exper_message_trait)
    Finished dev [unoptimized + debuginfo] target(s) in 0.64s
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
mysm: SmIndividualMsgsAny {
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

There are four benchmarks `sm_string_msgs`, `sm_enum_msgs`, `sm_enum_msgs_any` and `sm_individual_msgs_any`.
They similar in perforamnce except `sm_string_msgs` is about 20x slower. At this
time I wouldn't bet the house on any of these benchmarks, but its an indicator.

Note: See issue #1

```
$ taskset -c 0 cargo criterion
    Finished bench [optimized] target(s) in 0.03s
sm_string_msgs/sm_string_msgs                                                                             
                        time:   [879.76 ns 882.93 ns 886.74 ns]
                        change: [+0.0316% +0.3369% +0.7240%] (p = 0.05 < 0.05)
                        Change within noise threshold.

sm_enum_msgs/sm_enum_msgs                                                                             
                        time:   [43.476 ns 43.829 ns 44.225 ns]
                        change: [-0.6884% +0.5053% +1.8063%] (p = 0.42 > 0.05)
                        No change in performance detected.

sm_enum_msgs_any/sm_enum_msgs_any                                                                             
                        time:   [49.196 ns 49.446 ns 49.705 ns]
                        change: [-1.8809% -0.4704% +0.8184%] (p = 0.51 > 0.05)
                        No change in performance detected.

sm_individual_msgs_any/sm_individual_msgs_any                                                                             
                        time:   [38.654 ns 38.781 ns 38.961 ns]
                        change: [-2.4840% -2.0047% -1.5254%] (p = 0.00 < 0.05)
                        Performance has improved.


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
