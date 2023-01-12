use std::any::Any;

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
pub mod sm_individual_msgs_any;
