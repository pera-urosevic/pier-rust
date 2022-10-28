extern crate redis;

use crate::database::DB;
use crate::email;

use systemstat::Platform;

pub fn task() {
    let sys = systemstat::System::new();
    let memory = error_email!(sys.memory());
    let free = memory.free.as_u64() as f64;
    let total = memory.total.as_u64() as f64;
    let usage = 100 - ((free / total) * 100.0).round() as u64;
    // println!("[mem] {}", usage);
    DB::new().del("pier:mem").hset("pier:mem", "usage", usage);
}
