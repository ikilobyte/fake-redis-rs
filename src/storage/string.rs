use crate::protocol::{Lock, TTL};
use std::collections::HashMap;

// string类型
#[derive(Debug, Clone)]
pub struct TString {
    pub inner: HashMap<String, String>,
}

impl TString {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    // https://redis.io/commands/set
    pub fn set(
        &mut self,
        key: String,
        value: String,
        ttl: Option<(TTL, usize)>,
        lock: Option<Lock>,
    ) -> Result<String, ()> {
        println!("set {} {} ttl {:?} lock {:?} ", key, value, ttl, lock);

        Ok("+OK\r\n".to_string())
    }

    // https://redis.io/commands/get
    pub fn get(&self) -> Result<String, ()> {
        Ok("+OK\r\n".to_string())
    }
}
