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
}
