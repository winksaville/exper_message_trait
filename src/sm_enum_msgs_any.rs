use crate::{MsgAny, ProcessMsgAny};
use std::fmt::{self, Debug};

// Why do I have to declare a type alias here, I'd like to `use` it?
//    use crate::SmProcessMsgFn;
pub type SmProcessMsgFn<SM> = fn(&mut SM, Box<MsgAny>);

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum Messages {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

pub struct SmEnumMsgsAny {
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

impl Debug for SmEnumMsgsAny {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SmEnumMsgsAny")
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
impl SmEnumMsgsAny {
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

    pub fn state0(&mut self, msg: Box<MsgAny>) {
        self.state0_counter += 1;
        match msg.downcast_ref::<Messages>() {
            Some(Messages::Quit) => self.state0_quit_counter += 1,
            Some(Messages::Move { x, y }) => {
                self.state0_move_counter += 1;
                self.state0_move_xy_counter +=
                    x.unsigned_abs() as usize + y.unsigned_abs() as usize;
            }
            Some(Messages::Write(s)) => {
                self.state0_write_counter += 1;
                self.state0_write_sum_len_s += s.len();
            }
            None => self.state0_none_counter += 1,
        }

        self.transition(SmEnumMsgsAny::state1);
    }

    pub fn state1(&mut self, msg: Box<MsgAny>) {
        self.state1_counter += 1;
        match msg.downcast_ref::<Messages>() {
            Some(Messages::Quit) => self.state1_quit_counter += 1,
            Some(Messages::Move { x, y }) => {
                self.state1_move_counter += 1;
                self.state1_move_xy_counter +=
                    x.unsigned_abs() as usize + y.unsigned_abs() as usize;
            }
            Some(Messages::Write(s)) => {
                self.state1_write_counter += 1;
                self.state1_write_sum_len_s += s.len();
            }
            None => self.state1_none_counter += 1,
        }

        self.transition(SmEnumMsgsAny::state0);
    }
}

impl ProcessMsgAny for SmEnumMsgsAny {
    fn process_msg_any(&mut self, msg: Box<MsgAny>) {
        (self.current_state)(self, msg);
    }
}
