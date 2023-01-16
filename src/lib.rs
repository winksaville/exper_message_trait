use std::{any::Any, num::Wrapping};

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum EnumMsgs {
    Quit,
    Move { x: Wrapping<i32>, y: Wrapping<i32> },
    Write(String),
}

// Messages are things that implement trait std::any::Any
// which is most anything
pub type MsgAny = dyn Any;

// This type alias is generic and apparently can't be exported
// but Message can, oh well.
//pub type SmProcessMsgFn<SM> = fn(&mut SM, Box<Message>);

// Dispatch a message
pub trait ProcessMsgAny {
    fn process_msg_any(&mut self, msg: Box<MsgAny>);
}

pub mod sm_enum_msgs;
pub mod sm_enum_msgs_any;
pub mod sm_separate_msgs_any;
pub mod sm_string_msgs;
