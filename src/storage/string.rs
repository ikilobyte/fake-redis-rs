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
        // TODO 过期时间
        if let Some((ttl, second)) = ttl {
            println!("{:#?}", ttl);
            println!("{:#?}", second);
        }

        // TODO 处理锁
        if let Some(_) = lock {}
        self.inner.insert(key, value);

        Ok("+OK\r\n".to_string())
    }

    // https://redis.io/commands/get
    pub fn get(&self, key: String) -> Result<String, ()> {
        let resp = if let Some(val) = self.inner.get(&key) {
            format!("${}\r\n{}\r\n", val.len(), val)
        } else {
            // nil
            "$-1\r\n".to_string()
        };

        Ok(resp)
    }

    // 内部删除key
    pub fn remove(&mut self, key: &str) -> bool {
        if let Some(_) = self.inner.remove(key) {
            return true;
        }

        return false;
    }
}
