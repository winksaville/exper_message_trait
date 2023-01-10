use crossbeam_channel::{unbounded, Sender};
use std::{any::Any, time::Instant};

#[derive(Debug, Clone)]
pub struct PingReq {
    pub tx: Sender<Box<dyn Any>>,
    pub id: u64,
    pub instant: Instant,
}

impl PingReq {
    fn new(tx: Sender<Box<dyn Any>>, id: u64) -> Self {
        Self {
            tx,
            id,
            instant: Instant::now(),
        }
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
}

#[derive(Clone, Debug)]
pub struct Other;

impl Other {
    fn new() -> Other {
        Other
    }
}

trait DispatchMsg<SM> {
    fn dispatch_msg(&mut self, msg: Box<dyn Any>);
}

type ProcessMsgFn<SM> = fn(&mut SM, Box<dyn Any>) -> StateResult;

// I'd like this to be:
//   type StateResult<SM> = (Handled, Option<ProcessMsgFn<SM>>);
// But then StateResult is circular with ProcessMsgFn and it won't compile
type StateResult = Option<StateIdx>;
type StateIdx = usize;

#[allow(unused)]
pub struct MySm {
    states: Vec<ProcessMsgFn<Self>>,
    current_state: StateIdx,
    f1: i128,
}

impl MySm {
    pub fn new(f1: i128) -> Self {
        Self {
            states: vec![Self::state1, Self::state2],
            current_state: 0,
            f1,
        }
    }

    pub fn state1(&mut self, msg: Box<dyn Any>) -> StateResult {
        if let Some(msg) = msg.downcast_ref::<PingReq>() {
            println!("state1: msg={msg:?}");
            self.f1 += msg.id as i128;

            let reply_msg = PingRsp::new(2, msg);
            _ = msg.tx.send(Box::new(reply_msg));
        } else if let Some(msg) = msg.downcast_ref::<PingRsp>() {
            println!("state1: msg={msg:?}");
            self.f1 -= msg.id as i128;
        } else {
            println!("state1: Unknown msg type={:?}", msg.type_id());
        }

        Some(1) // Transition to state2, i.e. StateIdx 1
    }

    pub fn state2(&mut self, msg: Box<dyn Any>) -> StateResult {
        if let Some(msg) = msg.downcast_ref::<PingReq>() {
            println!("state2: msg={msg:?}");
            self.f1 -= msg.id as i128;

            let reply_msg = PingRsp::new(2, msg);
            _ = msg.tx.send(Box::new(reply_msg));
        } else if let Some(msg) = msg.downcast_ref::<PingRsp>() {
            println!("state2: msg={msg:?}");
            self.f1 -= msg.id as i128;
        } else {
            println!("state2: Unknown msg type={:?}", msg.type_id());
        }

        Some(0) // Transition to state1, i.e. StateIdx 0
    }
}

impl<MySM> DispatchMsg<MySM> for MySm {
    fn dispatch_msg(&mut self, msg: Box<dyn Any>) {
        let result = (self.states[self.current_state])(self, msg);
        if let Some(next_state_idx) = result {
            self.current_state = next_state_idx;
        }
    }
}

fn main() {
    let (tx, rx) = unbounded::<Box<dyn Any>>();

    let mut mysm = MySm::new(123);

    // Send/Recv PingReq dispatch
    let msg = PingReq::new(tx.clone(), 1);
    tx.send(Box::new(msg)).unwrap();
    let recv_msg = rx.recv().unwrap();
    <MySm as DispatchMsg<MySm>>::dispatch_msg(&mut mysm, recv_msg);

    // Recv PingRsp dispatch
    let recv_msg = rx.recv().unwrap();
    <MySm as DispatchMsg<MySm>>::dispatch_msg(&mut mysm, recv_msg);

    // Send/Recv second PingReq dispatch
    let msg = PingReq::new(tx.clone(), 1);
    tx.send(Box::new(msg)).unwrap();
    let recv_msg = rx.recv().unwrap();
    <MySm as DispatchMsg<MySm>>::dispatch_msg(&mut mysm, recv_msg);

    // Recv PingRsp dispatch
    let recv_msg = rx.recv().unwrap();
    <MySm as DispatchMsg<MySm>>::dispatch_msg(&mut mysm, recv_msg);

    // Send/Recv Other dispatch
    let other = Other::new();
    tx.send(Box::new(other)).unwrap();
    let recv_msg = rx.recv().unwrap();
    <MySm as DispatchMsg<MySm>>::dispatch_msg(&mut mysm, recv_msg);
}
