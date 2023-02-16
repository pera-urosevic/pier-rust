extern crate redis;

use crate::database::DB;
use crate::email;

use systemstat::Platform;

const SEC_HOUR: u64 = 60 * 60;
const SEC_DAY: u64 = SEC_HOUR * 24;

pub fn task() {
    let sys = systemstat::System::new();
    let uptime = error_email!(sys.uptime());
    let seconds = uptime.as_secs();
    let days = seconds / SEC_DAY;
    let hours = (seconds - days * SEC_DAY) / SEC_HOUR;
    // println!("[uptime] {}d {}h", days, hours);
    DB::new()
        .del("monitor:uptime")
        .hset("monitor:uptime", "days", days)
        .hset("monitor:uptime", "hours", hours);
}
