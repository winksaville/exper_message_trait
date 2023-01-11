use criterion::{criterion_group, criterion_main, Criterion, PlotConfiguration};
use exper_message_trait::{Message, ProcessMsg};

#[derive(Debug, Clone)]
enum Messages {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

pub type SmProcessMsgFn<SM> = fn(&mut SM, Box<Message>);

pub struct SmEnumMessages {
    current_state: SmProcessMsgFn<Self>,
    state0_quit_counter: usize,
    state0_move_counter: usize,
    state0_move_xy_counter: usize,
    state0_write_counter: usize,
    state0_write_sum_len_s: usize,
    state0_none_counter: usize,

    state1_quit_counter: usize,
    state1_move_counter: usize,
    state1_move_xy_counter: usize,
    state1_write_counter: usize,
    state1_write_sum_len_s: usize,
    state1_none_counter: usize,
}

impl SmEnumMessages {
    pub fn new(initial_state: SmProcessMsgFn<Self>) -> Self {
        Self {
            current_state: initial_state,
            state0_quit_counter: 0,
            state0_move_counter: 0,
            state0_move_xy_counter: 0,
            state0_write_counter: 0,
            state0_write_sum_len_s: 0,
            state0_none_counter: 0,

            state1_quit_counter: 0,
            state1_move_counter: 0,
            state1_move_xy_counter: 0,
            state1_write_counter: 0,
            state1_write_sum_len_s: 0,
            state1_none_counter: 0,
        }
    }

    fn transition(&mut self, dest: SmProcessMsgFn<Self>) {
        self.current_state = dest;
    }

    pub fn state0(&mut self, msg: Box<Message>) {
        match msg.downcast_ref::<Messages>() {
            Some(Messages::Quit) => self.state0_quit_counter += 1,
            Some(Messages::Move { x, y }) => {
                self.state0_move_counter += 1;
                self.state0_move_xy_counter += x.abs() as usize + y.abs() as usize;
            }
            Some(Messages::Write(s)) => {
                self.state0_write_counter += 1;
                self.state0_write_sum_len_s += s.len();
            }
            None => self.state0_none_counter += 1,
        }

        self.transition(SmEnumMessages::state1);
    }

    pub fn state1(&mut self, msg: Box<Message>) {
        match msg.downcast_ref::<Messages>() {
            Some(Messages::Quit) => self.state1_quit_counter += 1,
            Some(Messages::Move { x, y }) => {
                self.state1_move_counter += 1;
                self.state1_move_xy_counter += x.abs() as usize + y.abs() as usize;
            }
            Some(Messages::Write(s)) => {
                self.state1_write_counter += 1;
                self.state1_write_sum_len_s += s.len();
            }
            None => self.state1_none_counter += 1,
        }

        self.transition(SmEnumMessages::state0);
    }
}

impl ProcessMsg for SmEnumMessages {
    fn process_msg(&mut self, msg: Box<Message>) {
        (self.current_state)(self, msg);
    }
}

#[derive(Debug, Clone)]
struct Quit;

#[derive(Debug, Clone)]
struct Move {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Write(String);

pub struct SmIndividualMessages {
    current_state: SmProcessMsgFn<Self>,
    state0_quit_counter: usize,
    state0_move_counter: usize,
    state0_move_xy_counter: usize,
    state0_write_counter: usize,
    state0_write_sum_len_s: usize,
    state0_none_counter: usize,

    state1_quit_counter: usize,
    state1_move_counter: usize,
    state1_move_xy_counter: usize,
    state1_write_counter: usize,
    state1_write_sum_len_s: usize,
    state1_none_counter: usize,
}

impl SmIndividualMessages {
    pub fn new(initial_state: SmProcessMsgFn<Self>) -> Self {
        Self {
            current_state: initial_state,
            state0_quit_counter: 0,
            state0_move_counter: 0,
            state0_move_xy_counter: 0,
            state0_write_counter: 0,
            state0_write_sum_len_s: 0,
            state0_none_counter: 0,

            state1_quit_counter: 0,
            state1_move_counter: 0,
            state1_move_xy_counter: 0,
            state1_write_counter: 0,
            state1_write_sum_len_s: 0,
            state1_none_counter: 0,
        }
    }

    fn transition(&mut self, dest: SmProcessMsgFn<Self>) {
        self.current_state = dest;
    }

    pub fn state0(&mut self, msg: Box<Message>) {
        if let Some(_) = msg.downcast_ref::<Quit>() {
            self.state0_quit_counter += 1;
        } else if let Some(mm) = msg.downcast_ref::<Move>() {
            self.state0_move_counter += 1;
            self.state0_move_xy_counter += mm.x.abs() as usize + mm.y.abs() as usize;
        } else if let Some(mw) = msg.downcast_ref::<Write>() {
            self.state0_write_counter += 1;
            self.state0_write_sum_len_s += mw.0.len();
        } else {
            self.state0_none_counter += 1;
        }

        self.transition(SmIndividualMessages::state1);
    }

    pub fn state1(&mut self, msg: Box<Message>) {
        if let Some(_) = msg.downcast_ref::<Quit>() {
            self.state1_quit_counter += 1;
        } else if let Some(mm) = msg.downcast_ref::<Move>() {
            self.state1_move_counter += 1;
            self.state1_move_xy_counter += mm.x.abs() as usize + mm.y.abs() as usize;
        } else if let Some(mw) = msg.downcast_ref::<Write>() {
            self.state1_write_counter += 1;
            self.state1_write_sum_len_s += mw.0.len();
        } else {
            self.state1_none_counter += 1;
        }

        self.transition(SmIndividualMessages::state0);
    }
}

impl ProcessMsg for SmIndividualMessages {
    fn process_msg(&mut self, msg: Box<Message>) {
        (self.current_state)(self, msg);
    }
}

#[allow(unused)]
fn sm_enum_messages(c: &mut Criterion) {
    //println!("sm_enum_messages:+");

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

    //println!("sm_enum_messages:-");
}

#[allow(unused)]
fn sm_individual_messages(c: &mut Criterion) {
    //println!("sm_individual_messages:+");

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

    //println!("sm_enum_messages:-");
}

criterion_group!(benches, sm_enum_messages, sm_individual_messages);
//criterion_group!(benches, sm_individual_messages, sm_enum_messages);
criterion_main!(benches);
