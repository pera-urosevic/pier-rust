use dotenv::dotenv;
use std::env;
use std::thread;
use systemstat::Duration;

#[macro_use]
mod macros;

mod database;
mod email;
mod monitor;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    dotenv().ok();
    println!("pier {}", VERSION);
    let mut tasks = monitor::run();
    loop {
        tasks.run_pending();
        thread::sleep(Duration::from_millis(100));
    }
}
