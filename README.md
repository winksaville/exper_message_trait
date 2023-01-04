# Experiment with message trait

See if I can easily use a message trait to send
two different Types in one crossbeam_channel.

Added `trait Message`, this is a trivial trait just
to see that it was possible.

What I think I need to do is to get the "Type" of the message.
One option could be to use "std::any::Any" or use "TypeId",
more exploring to do!

See [src/main.rs](/src/main.rs)


## Run:

```
wink@3900x 23-01-03T23:30:37.607Z:~/prgs/rust/myrepos/exper_message_trait (main)
$ cargo run --release
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/exper_message_trait`
time between      instant1 and instant0 50
time between      instant2 and instant1 20
time between start_instant and instant2 20
time_to_send=1313ns time_to_recv=681ns travel_time=1994ns
(recv_instant - start_instant)=2014ns

time between      instant1 and instant0 20
time between      instant2 and instant1 20
time between start_instant and instant2 21
time_to_send=861ns time_to_recv=451ns travel_time=1312ns
(recv_instant - start_instant)=1332ns

time between      instant1 and instant0 20
time between      instant2 and instant1 20
time between start_instant and instant2 20
time_to_send=552ns time_to_recv=220ns travel_time=772ns
(recv_instant - start_instant)=792ns
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
