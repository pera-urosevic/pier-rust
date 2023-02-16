extern crate redis;

use redis::Commands;
use redis::Connection;
use redis::ToRedisArgs;

use crate::email;

pub struct DB {
    con: Connection,
}

impl DB {
    pub fn new() -> DB {
        let url = std::env::var("REDIS").expect("REDIS env");
        let client = error_email!(redis::Client::open(url));
        let con = error_email!(client.get_connection());
        DB { con }
    }

    pub fn del<K: ToRedisArgs>(mut self, key: K) -> Self {
        error_email!(self.con.del(key));
        self
    }

    pub fn hset<K: ToRedisArgs, F: ToRedisArgs, V: ToRedisArgs>(mut self, key: K, field: F, value: V) -> Self {
        error_email!(self.con.hset(key, field, value));
        self
    }

    pub fn counter(mut self, inc: bool, threshold: i32, key: &str, field: &str) -> bool {
        let mut counted = false;
        let mut value: i32 = match self.con.hget(key, field) {
            Ok(v) => v,
            Err(_) => 0,
        };
        if inc {
            value = value + 1;
        } else {
            value = 0;
        }
        // println!("inc {:#?}, value {:#?}", inc, value);
        if value >= threshold {
            counted = true;
            value = 0;
        }
        error_email!(self.con.hset(key, field, value));
        return counted;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        // counter will increase to 1
        let counted = DB::new().counter(true, 2, "test", "temp");
        assert_eq!(counted, false);
        // counter will reset to 0
        let counted = DB::new().counter(false, 2, "test", "temp");
        assert_eq!(counted, false);
        // counter will increase to 1
        let counted = DB::new().counter(true, 2, "test", "temp");
        assert_eq!(counted, false);
        // counter will hit threshold and reset to 0
        let counted = DB::new().counter(true, 2, "test", "temp");
        assert_eq!(counted, true);
        // counter will increase to 1
        let counted = DB::new().counter(true, 2, "test", "temp");
        assert_eq!(counted, false);
        // counter will hit threshold and reset to 0
        let counted = DB::new().counter(true, 2, "test", "temp");
        assert_eq!(counted, true);
        // cleanup
        DB::new().del("test");
    }
}
