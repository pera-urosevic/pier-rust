use clokwerk::{Scheduler, TimeUnits};

use crate::monitor;

pub fn run() -> Scheduler {
    println!("[services] start");
    let mut scheduler = Scheduler::new();

    if cfg!(debug_assertions) {
        // placeholder
    } else {
        monitor::uptime::task();
        monitor::storage::task();
        monitor::cpu::task();
        monitor::mem::task();
        monitor::swap::task();
        monitor::heartbeat::task();
        scheduler.every(1.hour()).run(|| monitor::uptime::task());
        scheduler.every(10.minute()).run(|| monitor::storage::task());
        scheduler.every(10.second()).run(|| monitor::cpu::task());
        scheduler.every(10.second()).run(|| monitor::mem::task());
        scheduler.every(10.second()).run(|| monitor::swap::task());
        scheduler.every(1.second()).run(|| monitor::heartbeat::task());
    }
    return scheduler;
}
