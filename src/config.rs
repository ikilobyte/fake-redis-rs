use crate::parser::{command, del, get, hdel, hget, hset, set};

use crate::protocol::Protocol;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref PARSE_LIST: HashMap<&'static str, fn(Vec<String>) -> Protocol> = {
        let mut map: HashMap<&str, fn(Vec<String>) -> Protocol> = HashMap::new();
        map.insert("SET", set::transform);
        map.insert("GET", get::transform);
        map.insert("HSET", hset::transform);
        map.insert("HGET", hget::transform);
        map.insert("HDEL", hdel::transform);
        map.insert("DEL", del::transform);
        map.insert("COMMAND", command::transform);
        map
    };
}
