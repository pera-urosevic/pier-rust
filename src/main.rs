use dotenv::dotenv;
use std::env;
use std::thread;
use systemstat::Duration;

#[macro_use]
mod macros;

mod database;
mod email;
mod monitor;
mod services;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    dotenv().ok();
    println!("pier {}", VERSION);
    let mut tasks = services::run();
    loop {
        tasks.run_pending();
        thread::sleep(Duration::from_millis(100));
    }
}
