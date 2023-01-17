use std::num::Wrapping;

use criterion::{criterion_group, criterion_main, Criterion, PlotConfiguration};

use exper_message_trait::{
    sm_enum_msgs::{ProcessMsg, SmEnumMsgs},
    sm_enum_msgs_any::SmEnumMsgsAny,
    sm_separate_msgs_any::{Move, Quit, SmSeparateMsgsAny, Write},
    sm_string_msgs::{ProcessStringMsg, SmStringMsgs},
    ProcessMsgAny,
};

use exper_message_trait::EnumMsgs;

#[allow(unused)]
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

#[allow(unused)]
fn quit_bench(c: &mut Criterion) {
    //println!("quit_bench:+");

    let plot_config = PlotConfiguration::default();

    let mut group = c.benchmark_group("quit");
    group.plot_config(plot_config);

    group.bench_function("separate_msgs_any", |b| {
        let mut sm = SmSeparateMsgsAny::new(SmSeparateMsgsAny::state0);

        b.iter(|| {
            let mq = Quit;
            let bmq = Box::new(mq);
            sm.process_msg_any(bmq);
        });
    });

    group.bench_function("enum_any", |b| {
        let mut sm = SmEnumMsgsAny::new(SmEnumMsgsAny::state0);

        b.iter(|| {
            let mq = EnumMsgs::Quit;
            let bmq = Box::new(mq);
            sm.process_msg_any(bmq);
        });
    });

    group.bench_function("enum_msgs", |b| {
        let mut sm = SmEnumMsgs::new(SmEnumMsgs::state0);

        b.iter(|| {
            let mq = EnumMsgs::Quit;
            let bmq = Box::new(mq);
            sm.process_msg(bmq);
        });
    });

    group.bench_function("string_msg", |b| {
        let mut sm = SmStringMsgs::new(SmStringMsgs::state0);

        b.iter(|| {
            let mq = String::from("Quit");
            sm.process_string_msg(mq);
        });
    });

    //println!("quit_bench:-");
}

#[allow(unused)]
fn write_bench(c: &mut Criterion) {
    //println!("write_bench:+");

    let plot_config = PlotConfiguration::default();

    let mut group = c.benchmark_group("write");
    group.plot_config(plot_config);

    group.bench_function("separate_msgs_any", |b| {
        let mut sm = SmSeparateMsgsAny::new(SmSeparateMsgsAny::state0);

        b.iter(|| {
            let mw = Write("Hi".to_owned());
            let bmw = Box::new(mw);
            sm.process_msg_any(bmw);
        });
    });

    group.bench_function("enum_any", |b| {
        let mut sm = SmEnumMsgsAny::new(SmEnumMsgsAny::state0);

        b.iter(|| {
            let mw = EnumMsgs::Write("Write Hello, world!".to_owned());
            let bmw = Box::new(mw);
            sm.process_msg_any(bmw);
        });
    });

    group.bench_function("enum_msgs", |b| {
        let mut sm = SmEnumMsgs::new(SmEnumMsgs::state0);

        b.iter(|| {
            let mw = EnumMsgs::Write("Write Hello, world!".to_owned());
            let bmw = Box::new(mw);
            sm.process_msg(bmw);
        });
    });

    group.bench_function("string_msg", |b| {
        let mut sm = SmStringMsgs::new(SmStringMsgs::state0);

        b.iter(|| {
            let mw = String::from("Write Hello, world!");
            sm.process_string_msg(mw);
        });
    });

    //println!("write_bench:-");
}

#[allow(unused)]
fn move_bench(c: &mut Criterion) {
    //println!("move_bench:+");

    let plot_config = PlotConfiguration::default();

    let mut group = c.benchmark_group("move");
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
        });
    });

    //println!("move_bench:-");
}

criterion_group!(benches, all_bench, quit_bench, write_bench, move_bench);
criterion_main!(benches);
