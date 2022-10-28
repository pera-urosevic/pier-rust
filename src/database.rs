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

    pub fn hset<K: ToRedisArgs, F: ToRedisArgs, V: ToRedisArgs>(
        mut self,
        key: K,
        field: F,
        value: V,
    ) -> Self {
        error_email!(self.con.hset(key, field, value));
        self
    }
}
