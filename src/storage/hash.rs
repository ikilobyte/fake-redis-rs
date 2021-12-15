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
}
