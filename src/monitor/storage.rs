extern crate redis;

use regex::Regex;
use std::num::ParseFloatError;

use crate::database::DB;
use crate::email;

use systemstat::Platform;

fn value(stat: String) -> Result<f64, ParseFloatError> {
    let value_string = stat.chars().take_while(|&ch| ch != ' ').collect::<String>();
    value_string.parse::<f64>()
}

pub fn task() {
    let storage_filter: String = std::env::var("STORAGE_FILTER").expect("STORAGE_FILTER env");
    let filter_regex: Regex = Regex::new(storage_filter.as_str()).expect("Storage RegExp valid");
    let sys = systemstat::System::new();
    let drives = error_email!(sys.mounts());
    let mut db = DB::new().del("pier:storage");
    for drive in drives {
        if filter_regex.is_match(&drive.fs_mounted_on) {
            let free = error_email!(value(drive.free.to_string()));
            let total = error_email!(value(drive.total.to_string()));
            let usage = 100 - (free / total * 100.0).round() as u64;
            let key = drive.fs_mounted_on;
            // println!("[storage] {} {}", key, usage);
            db = db.hset("pier:storage", key, usage);
        }
    }
    println!("done");
}
