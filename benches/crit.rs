use criterion::{criterion_group, criterion_main, Criterion, PlotConfiguration};

use exper_message_trait::{
    sm_enum_msgs::{Msgs, ProcessMsg, SmEnumMsgs},
    sm_enum_msgs_any::{Messages, SmEnumMsgsAny},
    sm_individual_msgs_any::{Move, Quit, SmIndividualMsgsAny, Write},
    ProcessMsgAny,
};

#[allow(unused)]
fn bench_sm_enum_msgs(c: &mut Criterion) {
    //println!("bench_sm_enum_msgs_any:+");

    let plot_config = PlotConfiguration::default();

    let mut group = c.benchmark_group("sm_enum_msgs");
    group.plot_config(plot_config);

    group.bench_function("sm_enum_msgs", |b| {
        //println!("bench:+");

        let mut sm = SmEnumMsgs::new(SmEnumMsgs::state0);

        let mm = Msgs::Move { x: 1, y: 2 };
        let mw = Msgs::Write("Hi".to_owned());
        let mq = Msgs::Quit;
        let bmm = Box::new(mm);
        let bmw = Box::new(mw);
        let bmq = Box::new(mq);

        b.iter(|| {
            //println!("b.iter:  Send Start");

            sm.process_msg(bmm.clone());
            sm.process_msg(bmw.clone());
            sm.process_msg(bmq.clone());
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

        let mm = Messages::Move { x: 1, y: 2 };
        let mw = Messages::Write("Hi".to_owned());
        let mq = Messages::Quit;
        let bmm = Box::new(mm);
        let bmw = Box::new(mw);
        let bmq = Box::new(mq);

        b.iter(|| {
            //println!("b.iter:  Send Start");

            sm.process_msg_any(bmm.clone());
            sm.process_msg_any(bmw.clone());
            sm.process_msg_any(bmq.clone());
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

        let mm = Move { x: 1, y: 2 };
        let mw = Write("Hi".to_owned());
        let mq = Quit;
        let bmm = Box::new(mm);
        let bmw = Box::new(mw);
        let bmq = Box::new(mq);

        b.iter(|| {
            //println!("b.iter:  Send Start");

            sm.process_msg_any(bmm.clone());
            sm.process_msg_any(bmw.clone());
            sm.process_msg_any(bmq.clone());
        });

        //println!("bench:-");
    });

    //println!("bench_sm_enum_msgs_any:-");
}

criterion_group!(
    benches,
    bench_sm_enum_msgs,
    bench_sm_enum_msgs_any,
    bench_sm_individual_msgs_any
);
criterion_main!(benches);
