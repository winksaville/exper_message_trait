use std::time::Instant;
use crossbeam_channel::unbounded;

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
}


fn main() {
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
}
