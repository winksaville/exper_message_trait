use std::any::Any;

// Messages are things that implement trait std::any::Any
// which is most anything
pub type Message = dyn Any;

// This type alias is generic and apparently can't be exported
// but Message can, oh well.
//pub type SmProcessMsgFn<SM> = fn(&mut SM, Box<Message>);

// Dispatch a message
pub trait ProcessMsg {
    fn process_msg(&mut self, msg: Box<Message>);
}

pub mod sm_enum_messages;
pub mod sm_individual_messages;
