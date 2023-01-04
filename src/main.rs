use std::{time::Instant, any::TypeId};
use core::mem::size_of_val;
//use crossbeam_channel::unbounded;

#[derive(Debug, Clone)]
pub struct PingReq {
    pub id: u64,
    pub instant: Instant,
}

impl PingReq {
    fn new(id: u64) -> Self {
        Self {
            id,
            instant: Instant::now(),
        }
    }

    fn print_size_of(&self) {
        println!("size_of_self={}", size_of_val(self));
    }

    #[allow(unused)]
    // Trying to use this in a `if {} else if {} else {}` statement
    // doesn't work because ba is moved and not returned. If ba is
    // a reference we still have a problem with because we then
    // need to clone to return the Option<Box<Self>>
    fn from_any(ba: Box<dyn std::any::Any>) -> Option<Box<Self>> {
        if let Ok(v) = ba.downcast::<Self>() {
            Some(v)
        } else {
            None
        }
    }

    fn as_boxed_any(self) -> Box<dyn std::any::Any + 'static> {
        Box::new(self)
    }
}

#[derive(Clone, Debug)]
pub struct PingRsp {
    pub req_id: u64,
    pub req_instant: Instant,
    pub id: u64,
    pub instant: Instant,
}

impl PingRsp {
    fn new(id: u64, req: &PingReq) -> PingRsp {
        Self {
            req_id: req.id,
            req_instant: req.instant,
            id,
            instant: Instant::now(),
        }
    }

    fn print_size_of(&self) {
        println!("size_of_self={}", size_of_val(self));
    }

    #[allow(unused)]
    // See above
    fn from_any(ba: Box<dyn std::any::Any>) -> Option<Box<Self>> {
        if let Ok(v) = ba.downcast::<Self>() {
            Some(v)
        } else {
            None
        }
    }

    fn as_boxed_any(self) -> Box<dyn std::any::Any + 'static> {
        Box::new(self)
    }
}


#[derive(Clone, Debug)]
pub struct Other;

impl Other {
    fn new() -> Other {
        Other
    }

    fn print_size_of(&self) {
        println!("size_of_self={}", size_of_val(self));
    }

    #[allow(unused)]
    // See above
    fn from_any(ba: Box<dyn std::any::Any>) -> Option<Box<Self>> {
        if let Ok(v) = ba.downcast::<Self>() {
            Some(v)
        } else {
            None
        }
    }

    fn as_boxed_any(self) -> Box<dyn std::any::Any + 'static> {
        Box::new(self)
    }
}

fn main() {
    let ping_req = PingReq::new(1);
    ping_req.print_size_of();

    let ping_rsp = PingRsp::new(1, &ping_req);
    ping_rsp.print_size_of();

    let other= Other::new();
    other.print_size_of();

    let ba_ping_req = ping_req.as_boxed_any();
    let ba_ping_rsp = ping_rsp.as_boxed_any();
    let ba_other = other.as_boxed_any();

    // From [here](https://doc.rust-lang.org/std/any/index.html#smart-pointers-and-dyn-any)
    // these should all be different
    println!("          ba_ping_req.type_id={:?}", ba_ping_req.type_id());
    println!("       (*ba_ping_req).type_id={:?}", (*ba_ping_req).type_id());
    println!("      (&*ba_ping_req).type_id={:?}", (&*ba_ping_req).type_id());

    // But they all return the same as this:
    println!("TypeId::of::<PingReg>={:?}", TypeId::of::<PingReq>());

    println!("          ba_ping_rsp.type_id={:?}", ba_ping_rsp.type_id());
    println!("       (*ba_ping_rsp).type_id={:?}", (*ba_ping_rsp).type_id());
    println!("      (&*ba_ping_rsp).type_id={:?}", (&*ba_ping_rsp).type_id());

    // But they all return the same as this:
    println!("TypeId::of::<PingRsp>={:?}", TypeId::of::<PingRsp>());

    println!("          ba_other.type_id={:?}", ba_other.type_id());
    println!("       (*ba_other).type_id={:?}", (*ba_other).type_id());
    println!("      (&*ba_other).type_id={:?}", (&*ba_other).type_id());

    // But they all return the same as this:
    println!("TypeId::of::<Other>={:?}", TypeId::of::<Other>());

    // So using Box<Any> via as_boxed_any() on the message allows me
    // to discriminates properly but we lose the Box and the
    // `if {} else fi {} else {}` doesn't look pretty.
    fn process_box_dyn_any(ba: &Box<dyn std::any::Any>) {
        // I couldn't get `from_any` working and this explanation
        // if probably wrong, but either way `from_any` isn't working for me.
        //
        // So, trying to use `from_any` doesn't work because we consume it:
        //    `if let Some(box_ping_req) = PingReq::from_any(ba_ping_req) {`
        // And trying to pass a reference doesn't work because we want to move it out :(
        //    `if let Some(box_ping_req) = PingReq::from_any(&ba_ping_req) {`
        // But doing downcast_ref we can handle any type of message, directly
        // here, but if we wan't to voe it we run into the same problem as
        // trying to to use `from_any`.
        if let Some(ping_req) = ba.downcast_ref::<PingReq>() {
            println!("Yes, ba: {ba:?} is a &PingReq: {ping_req:?}");
        } else if let Some(ping_rsp) = ba.downcast_ref::<PingRsp>() {
            println!("Yes, ba: {ba:?} is a &PingRsp: {ping_rsp:?}");
        } else {
            println!("Not, ping_req or ping_rsp");
        }
    }

    process_box_dyn_any(&ba_ping_req);
    process_box_dyn_any(&ba_ping_rsp);
    process_box_dyn_any(&ba_other);
}