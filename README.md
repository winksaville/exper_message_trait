# Experiment with message trait

See if I can easily use a message trait to send
two different Types in one crossbeam_channel.

Initially not using a trait and using two channels.

See [src/main.rs](/src/main.rs)

## Run:

```
wink@3900x 23-01-03T22:07:18.685Z:~/prgs/rust/myrepos/exper_message_trait (main)
$ cargo run --release
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/exper_message_trait`
time between      instant1 and instant0 50
time between      instant2 and instant1 30
time between start_instant and instant2 20
time_to_send=1433ns time_to_recv=741ns travel_time=2174ns
(recv_instant - start_instant)=2194ns
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
