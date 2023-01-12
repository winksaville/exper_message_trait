# Experiment with message trait

Based on a [ChatGPT conversation](https://chat.openai.com/chat/43127a48-08e0-4503-86eb-cb309ba89214).

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
 * [sm_enum_msgs.rs](/src/sm_enum_msgs.rs), msgs passed `as-is`
 * [sm_enum_msgs_any.rs](/src/sm_enum_msgs_any.rs), enum msgs passed via `dyn Any`.
 * [sm_individual_msgs_any.rs](/src/sm_individual_msgs_any.rs), individual `structs`'s passed via `dyn Any`.
 * [main.rs](/src/main.rs), a main which runs all three types.
 * [crit.rs](/benches/crit.rs), benchmarks.

## Run:

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/exper_message_trait`
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

There are thee benchmarks and `sm_enum_msgs`, `sm_enum_msgs_any` and `sm_individual_msgs_any`.
They similar in perforamnce except `sm_enum_msgs_any` is about 20% slower. At this
time I wouldn't bet the house on any of these benchmarks, but its an indicator.

Note: See issue #1

```
$ taskset -c 0 cargo criterion
   Compiling exper_message_trait v0.6.0 (/home/wink/prgs/rust/myrepos/exper_message_trait)
    Finished bench [optimized] target(s) in 4.43s
sm_enum_msgs/sm_enum_msgs                                                                             
                        time:   [39.594 ns 39.640 ns 39.703 ns]
                        change: [-2.8152% -1.8758% -1.0715%] (p = 0.00 < 0.05)
                        Performance has improved.

sm_enum_msgs_any/sm_enum_msgs_any                                                                             
                        time:   [47.733 ns 47.897 ns 48.054 ns]
                        change: [+4.4202% +6.6664% +8.4494%] (p = 0.00 < 0.05)
                        Performance has regressed.

sm_individual_msgs_any/sm_individual_msgs_any                                                                             
                        time:   [42.486 ns 42.540 ns 42.597 ns]
                        change: [+1.8950% +2.6158% +3.1267%] (p = 0.00 < 0.05)
                        Performance has regressed.


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
