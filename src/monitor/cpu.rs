extern crate redis;

use std::thread;

use crate::database::DB;

use systemstat::{Duration, Platform};

use super::alert::alert;

fn temp() {
    let sys = systemstat::System::new();
    match sys.cpu_temp() {
        Ok(temp) => {
            // println!("[cpu] temp {}", temp);
            alert(1, temp > 70.0, "cpu temp", format!("{}°C", temp).as_str());
            DB::new().hset("monitor:cpu", "temp", temp);
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
                    alert(3, usage > 90.0, "cpu usage", format!("{}%", usage).as_str());
                    DB::new().hset("monitor:cpu", "usage", usage);
                }
                Err(error) => println!("[cpu] usage {}", error),
            };
        }
        Err(error) => println!("[cpu] usage {}", error),
    }
}

pub fn task() {
    DB::new().del("monitor:cpu");
    temp();
    usage();
}
