use clokwerk::{Scheduler, TimeUnits};

mod alert;
mod cpu;
mod mem;
mod storage;
mod swap;
mod uptime;

pub fn run() -> Scheduler {
    println!("[monitor] start");
    let mut scheduler = Scheduler::new();
    uptime::task();
    storage::task();
    cpu::task();
    mem::task();
    swap::task();
    scheduler.every(1.hour()).run(|| uptime::task());
    scheduler.every(10.minute()).run(|| storage::task());
    scheduler.every(10.second()).run(|| cpu::task());
    scheduler.every(10.second()).run(|| mem::task());
    scheduler.every(10.second()).run(|| swap::task());
    return scheduler;
}
