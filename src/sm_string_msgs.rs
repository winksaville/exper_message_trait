use std::{
    error::Error,
    fmt::{self, Debug},
    num::Wrapping,
};

// Dispatch a message
pub trait ProcessStringMsg {
    fn process_string_msg(&mut self, msg: String);
}

pub type SmProcessMsgFn<SM> = fn(&mut SM, String);

// From: https://docs.rs/once_cell/1.17.0/once_cell/#lazily-compiled-regex
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

fn decode_move(msg: String) -> Result<(Wrapping<i32>, Wrapping<i32>), Box<dyn Error>> {
    let re = regex!(r"Move:\s*\{\s*x:\s*(\d+)\s*,\s*y:\s*(\d+)\s*\}");
    if let Some(cap) = re.captures(&msg) {
        if cap.len() == 3 {
            let x = Wrapping::<i32>(cap[1].parse()?);
            let y = Wrapping::<i32>(cap[2].parse()?);

            Ok((x, y))
        } else {
            panic!("Impossible or re: '{re:?}', expecting 2 captured items in Move: msg: '{msg}'");
        }
    } else {
        return Err(format!("Bad Move msg: '{msg}'").into());
    }
}

fn decode_write(msg: String) -> Result<String, Box<dyn Error>> {
    let re = regex!(r"Write:\s*(.*)");
    if let Some(cap) = re.captures(&msg) {
        if cap.len() == 2 {
            Ok(cap[1].to_owned())
        } else {
            panic!("Impossible or re: '{re:?}', expecting 1 captured items in Write: msg: '{msg}'");
        }
    } else {
        return Err(format!("Bad Write msg: '{msg}'").into());
    }
}

pub struct SmStringMsgs {
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

impl Debug for SmStringMsgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SmStringMsgs")
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
impl SmStringMsgs {
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
    pub fn state0(&mut self, msg: String) {
        self.state0_counter += 1;
        if msg.starts_with("Quit:") {
            self.state0_quit_counter += 1;
        } else if msg.starts_with("Move:") {
            match decode_move(msg) {
                Ok((x, y)) => {
                    self.state0_move_counter += 1;
                    self.state0_move_xy_counter += x + y;
                }
                Err(why) => {
                    panic!("state0: {why}");
                }
            }
        } else if msg.starts_with("Write:") {
            match decode_write(msg) {
                Ok(s) => {
                    self.state0_write_counter += 1;
                    self.state0_write_sum_len_s += s.len();
                }
                Err(why) => {
                    panic!("state0: {why}");
                }
            }
        } else {
            self.state0_none_counter += 1;
        }

        self.transition(SmStringMsgs::state1);
    }

    #[allow(clippy::boxed_local)]
    pub fn state1(&mut self, msg: String) {
        self.state1_counter += 1;
        if msg.starts_with("Quit:") {
            self.state1_quit_counter += 1;
        } else if msg.starts_with("Move:") {
            match decode_move(msg) {
                Ok((x, y)) => {
                    self.state1_move_counter += 1;
                    self.state1_move_xy_counter += x + y;
                }
                Err(why) => {
                    panic!("state1: {why}");
                }
            }
        } else if msg.starts_with("Write:") {
            match decode_write(msg) {
                Ok(s) => {
                    self.state1_write_counter += 1;
                    self.state1_write_sum_len_s += s.len();
                }
                Err(why) => {
                    panic!("state1: {why}");
                }
            }
        } else {
            self.state1_none_counter += 1;
        }

        self.transition(SmStringMsgs::state0);
    }
}

impl ProcessStringMsg for SmStringMsgs {
    fn process_string_msg(&mut self, msg: String) {
        (self.current_state)(self, msg);
    }
}
