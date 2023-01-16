use std::{
    fmt::{self, Debug},
    num::Wrapping,
};

use crate::EnumMsgs;

// Dispatch a message
pub trait ProcessMsg<Msg> {
    fn process_msg(&mut self, msg: Box<Msg>);
}

pub type SmProcessMsgFn<SM> = fn(&mut SM, Box<EnumMsgs>);

pub struct SmEnumMsgs {
    current_state: SmProcessMsgFn<Self>,
    pub state0_counter: usize,
    pub state0_quit_counter: usize,
    pub state0_move_counter: usize,
    pub state0_move_xy_counter: Wrapping<i32>,
    pub state0_write_counter: usize,
    pub state0_write_sum_len_s: usize,
    pub state0_none_counter: usize,

    pub state1_counter: usize,
    pub state1_quit_counter: usize,
    pub state1_move_counter: usize,
    pub state1_move_xy_counter: Wrapping<i32>,
    pub state1_write_counter: usize,
    pub state1_write_sum_len_s: usize,
    pub state1_none_counter: usize,
}

impl Debug for SmEnumMsgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SmEnumMsgs")
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
impl SmEnumMsgs {
    pub fn new(initial_state: SmProcessMsgFn<Self>) -> Self {
        Self {
            current_state: initial_state,
            state0_counter: 0,
            state0_quit_counter: 0,
            state0_move_counter: 0,
            state0_move_xy_counter: Wrapping(0),
            state0_write_counter: 0,
            state0_write_sum_len_s: 0,
            state0_none_counter: 0,

            state1_counter: 0,
            state1_quit_counter: 0,
            state1_move_counter: 0,
            state1_move_xy_counter: Wrapping(0),
            state1_write_counter: 0,
            state1_write_sum_len_s: 0,
            state1_none_counter: 0,
        }
    }

    fn transition(&mut self, dest: SmProcessMsgFn<Self>) {
        self.current_state = dest;
    }

    #[allow(clippy::boxed_local)]
    pub fn state0(&mut self, msg: Box<EnumMsgs>) {
        self.state0_counter += 1;
        match *msg {
            EnumMsgs::Quit => self.state0_quit_counter += 1,
            EnumMsgs::Move { x, y } => {
                self.state0_move_counter += 1;
                self.state0_move_xy_counter += x + y;
            }
            EnumMsgs::Write(s) => {
                self.state0_write_counter += 1;
                self.state0_write_sum_len_s += s.len();
            }
        }

        self.transition(SmEnumMsgs::state1);
    }

    #[allow(clippy::boxed_local)]
    pub fn state1(&mut self, msg: Box<EnumMsgs>) {
        self.state1_counter += 1;
        match *msg {
            EnumMsgs::Quit => self.state1_quit_counter += 1,
            EnumMsgs::Move { x, y } => {
                self.state1_move_counter += 1;
                self.state1_move_xy_counter += x + y;
            }
            EnumMsgs::Write(s) => {
                self.state1_write_counter += 1;
                self.state1_write_sum_len_s += s.len();
            }
        }

        self.transition(SmEnumMsgs::state0);
    }
}

impl ProcessMsg<EnumMsgs> for SmEnumMsgs {
    fn process_msg(&mut self, msg: Box<EnumMsgs>) {
        (self.current_state)(self, msg);
    }
}
