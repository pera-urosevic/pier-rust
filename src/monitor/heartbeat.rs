extern crate redis;

use std::time::{SystemTime, UNIX_EPOCH};

use crate::database::DB;

pub fn task() {
    let time = SystemTime::now();
    let ts = time.duration_since(UNIX_EPOCH).expect("Time travel not possible");
    let timestamp = format!("{}", ts.as_millis());
    // println!("[heartbeat] {}", timestamp);
    DB::new()
        .del("monitor:heartbeat")
        .hset("monitor:heartbeat", "timestamp", timestamp);
}
