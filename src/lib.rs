use std::any::Any;

// Messages are things that implement trait std::any::Any
// which is most anything
pub type Message = dyn Any;

// Dispatch a message
pub trait ProcessMsg {
    fn process_msg(&mut self, msg: Box<Message>);
}
