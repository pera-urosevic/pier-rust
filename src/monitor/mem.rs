extern crate redis;

use crate::database::DB;
use crate::email;

use systemstat::Platform;

use super::alert::alert;

pub fn task() {
    let sys = systemstat::System::new();
    let memory = error_email!(sys.memory());
    let free = memory.free.as_u64() as f64;
    let total = memory.total.as_u64() as f64;
    let usage = 100.0 - ((free / total) * 100.0);
    // println!("[mem] {}", usage);
    alert(2, usage > 90.0, "mem usage", format!("{}%", usage).as_str());
    DB::new().del("pier:mem").hset("pier:mem", "usage", usage);
}
