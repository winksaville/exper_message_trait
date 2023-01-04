use std::time::Instant;
use crossbeam_channel::unbounded;

pub trait Message {
    fn get_id(&self) -> u64;
    fn get_instant(&self) -> Instant;
    fn get_req_id(&self) -> u64;
    fn get_req_instant(&self) -> Instant;
}

#[derive(Clone)]
pub struct PingReq {
    pub id: u64,
    pub instant: Instant,
}

impl PingReq {
    fn new(id: u64) -> PingReq {
        Self {
            id,
            instant: Instant::now(),
        }
    }
}

impl Message for PingReq {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_instant(&self) -> Instant {
        self.instant
    }

    fn get_req_id(&self) -> u64 {
        self.id
    }

    fn get_req_instant(&self) -> Instant {
        self.instant
    }
}

#[derive(Clone)]
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
    fn from(id: u64, req: Box<dyn Message>) -> PingRsp {
        Self {
            req_id: req.get_id(),
            req_instant: req.get_instant(),
            id,
            instant: Instant::now(),
        }
    }
}

impl Message for PingRsp {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_instant(&self) -> Instant {
        self.instant
    }

    fn get_req_id(&self) -> u64 {
        self.req_id
    }

    fn get_req_instant(&self) -> Instant {
        self.req_instant
    }
}

fn two_channels() {
    let (ping_req_tx, ping_req_rx) = unbounded::<PingReq>();
    let (ping_rsp_tx, ping_rsp_rx) = unbounded::<PingRsp>();

    let instant0 = Instant::now();
    let instant1 = Instant::now();
    let instant2 = Instant::now();
    let start_instant = Instant::now();

    // Create req and send it
    let req = PingReq::new(1);
    ping_req_tx.send(req).unwrap();

    // Receive response
    let req_recv = ping_req_rx.recv().unwrap();

    // Send response
    let rsp = PingRsp::new(2, &req_recv);
    ping_rsp_tx.send(rsp).unwrap();

    // Recv response
    let rsp_recv = ping_rsp_rx.recv().unwrap();
    let recv_instant = Instant::now();

    let time_to_send= rsp_recv.instant.duration_since(rsp_recv.req_instant).as_nanos();
    let time_to_recv= recv_instant.duration_since(rsp_recv.instant).as_nanos();
    let travel_time = recv_instant.duration_since(rsp_recv.req_instant).as_nanos();
    let main_rtt = recv_instant.duration_since(start_instant).as_nanos();

    println!("time between      instant1 and instant0 {}", instant1.duration_since(instant0).as_nanos());
    println!("time between      instant2 and instant1 {}", instant2.duration_since(instant1).as_nanos());
    println!("time between start_instant and instant2 {}", start_instant.duration_since(instant2).as_nanos());
    println!("time_to_send={time_to_send}ns time_to_recv={time_to_recv}ns travel_time={travel_time}ns");
    println!("(recv_instant - start_instant)={}ns", main_rtt);
    println!();
}

fn two_channels_using_boxes() {
    let (ping_req_tx, ping_req_rx) = unbounded::<Box<PingReq>>();
    let (ping_rsp_tx, ping_rsp_rx) = unbounded::<Box<PingRsp>>();

    let instant0 = Instant::now();
    let instant1 = Instant::now();
    let instant2 = Instant::now();
    let start_instant = Instant::now();

    // Create req and send it
    let req = Box::new(PingReq::new(1));
    ping_req_tx.send(req).unwrap();

    // Receive response
    let req_recv = ping_req_rx.recv().unwrap();

    // Send response
    let rsp = Box::new(PingRsp::new(2, &req_recv));
    ping_rsp_tx.send(rsp).unwrap();

    // Recv response
    let rsp_recv = ping_rsp_rx.recv().unwrap();
    let recv_instant = Instant::now();

    let time_to_send= rsp_recv.instant.duration_since(rsp_recv.req_instant).as_nanos();
    let time_to_recv= recv_instant.duration_since(rsp_recv.instant).as_nanos();
    let travel_time = recv_instant.duration_since(rsp_recv.req_instant).as_nanos();
    let main_rtt = recv_instant.duration_since(start_instant).as_nanos();

    println!("time between      instant1 and instant0 {}", instant1.duration_since(instant0).as_nanos());
    println!("time between      instant2 and instant1 {}", instant2.duration_since(instant1).as_nanos());
    println!("time between start_instant and instant2 {}", start_instant.duration_since(instant2).as_nanos());
    println!("time_to_send={time_to_send}ns time_to_recv={time_to_recv}ns travel_time={travel_time}ns");
    println!("(recv_instant - start_instant)={}ns", main_rtt);
    println!();
}

fn one_channel() {
    let (tx, rx) = unbounded::<Box<dyn Message>>();

    let instant0 = Instant::now();
    let instant1 = Instant::now();
    let instant2 = Instant::now();
    let start_instant = Instant::now();

    // Create req and send it
    let req = Box::new(PingReq::new(1));
    tx.send(req).unwrap();

    // Receive response
    let req_recv = rx.recv().unwrap();

    // Send response
    let rsp = Box::new(PingRsp::from(2, req_recv));
    tx.send(rsp).unwrap();

    // Recv response
    let rsp_recv = rx.recv().unwrap();
    let recv_instant = Instant::now();

    let time_to_send= rsp_recv.get_instant().duration_since(rsp_recv.get_req_instant()).as_nanos();
    let time_to_recv= recv_instant.duration_since(rsp_recv.get_instant()).as_nanos();
    let travel_time = recv_instant.duration_since(rsp_recv.get_req_instant()).as_nanos();
    let main_rtt = recv_instant.duration_since(start_instant).as_nanos();

    println!("time between      instant1 and instant0 {}", instant1.duration_since(instant0).as_nanos());
    println!("time between      instant2 and instant1 {}", instant2.duration_since(instant1).as_nanos());
    println!("time between start_instant and instant2 {}", start_instant.duration_since(instant2).as_nanos());
    println!("time_to_send={time_to_send}ns time_to_recv={time_to_recv}ns travel_time={travel_time}ns");
    println!("(recv_instant - start_instant)={}ns", main_rtt);
    println!();
}

fn main() {
    two_channels();
    two_channels_using_boxes();
    one_channel();
}
