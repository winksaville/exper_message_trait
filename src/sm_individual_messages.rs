use crate::{Message, ProcessMsg};
use std::fmt::{self, Debug};

// Why do I have to declare a type alias here, I'd like to `use` it?
//    use crate::SmProcessMsgFn;
pub type SmProcessMsgFn<SM> = fn(&mut SM, Box<Message>);

#[derive(Debug, Clone)]
pub struct Quit;

#[derive(Debug, Clone)]
pub struct Move {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct Write(pub String);

//#[derive(Debug)]
pub struct SmIndividualMessages {
    current_state: SmProcessMsgFn<Self>,
    pub state0_counter: usize,
    pub state0_quit_counter: usize,
    pub state0_move_counter: usize,
    pub state0_move_xy_counter: usize,
    pub state0_write_counter: usize,
    pub state0_write_sum_len_s: usize,
    pub state0_none_counter: usize,

    pub state1_counter: usize,
    pub state1_quit_counter: usize,
    pub state1_move_counter: usize,
    pub state1_move_xy_counter: usize,
    pub state1_write_counter: usize,
    pub state1_write_sum_len_s: usize,
    pub state1_none_counter: usize,
}

impl Debug for SmIndividualMessages {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SmIndividualMessages")
            //.field("current_state", &self.current_state)
            .field("state0_counter", &self.state0_counter)
            .field("state0_quit_counter", &self.state0_quit_counter)
            .field("state0_move_counter", &self.state0_move_counter)
            .field("state0_move_xy_counter", &self.state0_move_xy_counter)
            .field("state0_write_counter", &self.state0_write_counter)
            .field(
                "state0_write_sum_len_s_counter",
                &self.state0_write_sum_len_s,
            )
            .field("state0_none_counter", &self.state0_none_counter)
            .field("state1_counter", &self.state1_counter)
            .field("state1_quit_counter", &self.state1_quit_counter)
            .field("state1_move_counter", &self.state1_move_counter)
            .field("state1_move_xy_counter", &self.state1_move_xy_counter)
            .field("state1_write_counter", &self.state1_write_counter)
            .field(
                "state1_write_sum_len_s_counter",
                &self.state1_write_sum_len_s,
            )
            .field("state1_none_counter", &self.state1_none_counter)
            .finish()
    }
}

#[allow(unused)]
impl SmIndividualMessages {
    pub fn new(initial_state: SmProcessMsgFn<Self>) -> Self {
        Self {
            current_state: initial_state,
            state0_counter: 0,
            state0_quit_counter: 0,
            state0_move_counter: 0,
            state0_move_xy_counter: 0,
            state0_write_counter: 0,
            state0_write_sum_len_s: 0,
            state0_none_counter: 0,

            state1_counter: 0,
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
        self.state0_counter += 1;
        if msg.downcast_ref::<Quit>().is_some() {
            self.state0_quit_counter += 1;
        } else if let Some(mm) = msg.downcast_ref::<Move>() {
            self.state0_move_counter += 1;
            self.state0_move_xy_counter +=
                mm.x.unsigned_abs() as usize + mm.y.unsigned_abs() as usize;
        } else if let Some(mw) = msg.downcast_ref::<Write>() {
            self.state0_write_counter += 1;
            self.state0_write_sum_len_s += mw.0.len();
        } else {
            self.state0_none_counter += 1;
        }

        self.transition(SmIndividualMessages::state1);
    }

    pub fn state1(&mut self, msg: Box<Message>) {
        self.state1_counter += 1;
        if msg.downcast_ref::<Quit>().is_some() {
            self.state1_quit_counter += 1;
        } else if let Some(mm) = msg.downcast_ref::<Move>() {
            self.state1_move_counter += 1;
            self.state1_move_xy_counter +=
                mm.x.unsigned_abs() as usize + mm.y.unsigned_abs() as usize;
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
