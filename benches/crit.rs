use std::num::Wrapping;

use criterion::{criterion_group, criterion_main, Criterion, PlotConfiguration};

use exper_message_trait::{
    sm_enum_msgs::{Msgs, ProcessMsg, SmEnumMsgs},
    sm_enum_msgs_any::{Messages, SmEnumMsgsAny},
    sm_individual_msgs_any::{Move, Quit, SmIndividualMsgsAny, Write},
    sm_string_msgs::{ProcessStringMsg, SmStringMsgs},
    ProcessMsgAny,
};

#[allow(unused)]
fn bench_sm_string_msgs(c: &mut Criterion) {
    //println!("bench_sm_string_msgs_any:+");

    let plot_config = PlotConfiguration::default();

    let mut group = c.benchmark_group("sm_string_msgs");
    group.plot_config(plot_config);

    group.bench_function("sm_string_msgs", |b| {
        //println!("bench:+");

        let mut sm = SmStringMsgs::new(SmStringMsgs::state0);

        let mut x = Wrapping(1i32);
        let mut y = Wrapping(2i32);
        b.iter(|| {
            x += 1;
            y += 1;
            let bmm = String::from("Write: Hello, world!");
            sm.process_string_msg(bmm);

            let bmw = format!("Move:{{x:{},y:{}}}", 1, 2);
            sm.process_string_msg(bmw);

            let bmq = String::from("Quit");
            sm.process_string_msg(bmq);
        });
        //println!("bench:-");
    });

    //println!("bench_sm_string_msgs_any:-");
}

#[allow(unused)]
fn bench_sm_enum_msgs(c: &mut Criterion) {
    //println!("bench_sm_enum_msgs_any:+");

    let plot_config = PlotConfiguration::default();

    let mut group = c.benchmark_group("sm_enum_msgs");
    group.plot_config(plot_config);

    group.bench_function("sm_enum_msgs", |b| {
        //println!("bench:+");

        let mut sm = SmEnumMsgs::new(SmEnumMsgs::state0);

        let mut x = Wrapping(1i32);
        let mut y = Wrapping(2i32);
        b.iter(|| {
            x += 1;
            y += 1;
            let mm = Msgs::Move { x, y };
            let bmm = Box::new(mm);
            sm.process_msg(bmm);

            let mw = Msgs::Write("Hi".to_owned());
            let bmw = Box::new(mw);
            sm.process_msg(bmw);

            let mq = Msgs::Quit;
            let bmq = Box::new(mq);
            sm.process_msg(bmq);
        });
        //println!("bench:-");
    });

    //println!("bench_sm_enum_msgs_any:-");
}

#[allow(unused)]
fn bench_sm_enum_msgs_any(c: &mut Criterion) {
    //println!("bench_sm_enum_msgs_any:+");

    let plot_config = PlotConfiguration::default();

    let mut group = c.benchmark_group("sm_enum_msgs_any");
    group.plot_config(plot_config);

    group.bench_function("sm_enum_msgs_any", |b| {
        //println!("bench:+");

        let mut sm = SmEnumMsgsAny::new(SmEnumMsgsAny::state0);

        let mut x = Wrapping(1i32);
        let mut y = Wrapping(2i32);
        b.iter(|| {
            x += 1;
            y += 1;
            let mm = Messages::Move { x, y };
            let bmm = Box::new(mm);
            sm.process_msg_any(bmm);

            let mw = Messages::Write("Hi".to_owned());
            let bmw = Box::new(mw);
            sm.process_msg_any(bmw);

            let mq = Messages::Quit;
            let bmq = Box::new(mq);
            sm.process_msg_any(bmq);
        });

        //println!("bench:-");
    });

    //println!("bench_sm_enum_msgs_any:-");
}

#[allow(unused)]
fn bench_sm_individual_msgs_any(c: &mut Criterion) {
    //println!("bench_sm_individual_msgs_any:+");

    let plot_config = PlotConfiguration::default();

    let mut group = c.benchmark_group("sm_individual_msgs_any");
    group.plot_config(plot_config);

    group.bench_function("sm_individual_msgs_any", |b| {
        //println!("bench:+");

        let mut sm = SmIndividualMsgsAny::new(SmIndividualMsgsAny::state0);

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

        //println!("bench:-");
    });

    //println!("bench_sm_enum_msgs_any:-");
}

criterion_group!(
    benches,
    bench_sm_string_msgs,
    bench_sm_enum_msgs,
    bench_sm_enum_msgs_any,
    bench_sm_individual_msgs_any
);
criterion_main!(benches);
