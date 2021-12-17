use std::collections::HashMap;

type HashData = HashMap<String, HashMap<String, String>>;

// hash类型
#[derive(Debug, Clone)]
pub struct THash {
    pub inner: HashData,
}

impl THash {
    pub fn new() -> Self {
        Self {
            inner: HashData::new(),
            // observer: String,
        }
    }

    // https://redis.io/commands/hset
    pub fn set(&mut self, key: String, field: String, value: String) -> Result<String, ()> {
        let mut map = if let Some(x) = self.inner.get_mut(&key) {
            x.clone()
        } else {
            HashMap::new()
        };

        let mut count = 0;
        if let None = map.insert(field, value) {
            count += 1;
        }
        self.inner.insert(key, map);

        Ok(format!(":{}\r\n", count).to_string())
    }

    // https://redis.io/commands/hget
    pub fn get(&self, key: String, field: String) -> Result<String, ()> {
        let resp = match self.inner.get(&key) {
            None => format!("$-1\r\n"),
            Some(map) => match map.get(&field) {
                None => format!("$-1\r\n"),
                Some(x) => format!("${}\r\n{}\r\n", x.len(), x),
            },
        };

        Ok(resp)
    }

    // https://redis.io/commands/hdel
    pub fn del(&mut self, key: String, fields: Vec<String>) -> Result<String, ()> {
        let mut count = 0;

        let resp = match self.inner.get_mut(&key) {
            None => format!(":0\r\n"),
            Some(map) => {
                if map.len() <= 0 {
                    format!(":0\r\n")
                } else {
                    for field in fields.iter() {
                        if let Some(_) = map.remove(field) {
                            count += 1;
                        }
                    }

                    // 删除最外层的key
                    if map.len() <= 0 {
                        self.remove(&key);
                    }

                    format!(":{}\r\n", count)
                }
            }
        };

        Ok(resp)
    }

    // 内部删除
    pub fn remove(&mut self, key: &str) -> bool {
        if let Some(_) = self.inner.remove(key) {
            return true;
        }
        false
    }
}
