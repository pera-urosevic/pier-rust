extern crate redis;

use crate::database::DB;
use crate::email;

use systemstat::Platform;

pub fn task() {
    let sys = systemstat::System::new();
    let swap = error_email!(sys.swap());
    let free = swap.free.as_u64() as f64;
    let total = swap.total.as_u64() as f64;
    let usage = 100 - ((free / total) * 100.0).round() as u64;
    // println!("[swap] {}", usage);
    DB::new().del("monitor:swap").hset("monitor:swap", "usage", usage);
}
