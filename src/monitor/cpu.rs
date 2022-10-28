extern crate redis;

use std::thread;

use crate::database::DB;

use systemstat::{Duration, Platform};

fn temp() {
    let sys = systemstat::System::new();
    match sys.cpu_temp() {
        Ok(temp) => {
            // println!("[cpu] temp {}", temp);
            DB::new().hset("pier:cpu", "temp", temp);
        }
        Err(error) => {
            println!("[cpu] temp {}", error);
        }
    };
}

fn usage() {
    let sys = systemstat::System::new();
    match sys.cpu_load_aggregate() {
        Ok(request) => {
            thread::sleep(Duration::from_secs(1));
            match request.done() {
                Ok(cpu) => {
                    let usage = (cpu.user + cpu.nice + cpu.system + cpu.interrupt) * 100.0;
                    // println!("[cpu] usage {}", usage);
                    DB::new().hset("pier:cpu", "usage", usage);
                }
                Err(error) => println!("[cpu] usage {}", error),
            };
        }
        Err(error) => println!("[cpu] usage {}", error),
    }
}

pub fn task() {
    DB::new().del("pier:cpu");
    temp();
    usage();
}
