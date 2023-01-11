use criterion::{criterion_group, criterion_main, Criterion, PlotConfiguration};

use exper_message_trait::{
    sm_enum_messages::{Messages, SmEnumMessages},
    sm_individual_messages::{Move, Quit, SmIndividualMessages, Write},
    ProcessMsg,
};

#[allow(unused)]
fn bench_sm_enum_messages(c: &mut Criterion) {
    //println!("bench_sm_enum_messages:+");

    let plot_config = PlotConfiguration::default();

    let mut group = c.benchmark_group("sm_enum_messages");
    group.plot_config(plot_config);

    group.bench_function("sm_enum_messages", |b| {
        //println!("bench:+");

        let mut sm = SmEnumMessages::new(SmEnumMessages::state0);

        let mm = Messages::Move { x: 1, y: 2 };
        let mw = Messages::Write("Hi".to_owned());
        let mq = Messages::Quit;
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

    //println!("bench_sm_enum_messages:-");
}

#[allow(unused)]
fn bench_sm_individual_messages(c: &mut Criterion) {
    //println!("bench_sm_individual_messages:+");

    let plot_config = PlotConfiguration::default();

    let mut group = c.benchmark_group("sm_individual_messages");
    group.plot_config(plot_config);

    group.bench_function("sm_individual_messages", |b| {
        //println!("bench:+");

        let mut sm = SmIndividualMessages::new(SmIndividualMessages::state0);

        let mm = Move { x: 1, y: 2 };
        let mw = Write("Hi".to_owned());
        let mq = Quit;
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

    //println!("bench_sm_enum_messages:-");
}

criterion_group!(
    benches,
    bench_sm_enum_messages,
    bench_sm_individual_messages
);
criterion_main!(benches);
