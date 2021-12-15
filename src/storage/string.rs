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
    pub fn set(&mut self) -> Result<&[u8], ()> {
        let x = b"+Ok\r\n";
        Ok(x)
    }

    // https://redis.io/commands/get
    pub fn get(&self) -> Result<&[u8], ()> {
        Ok(b"xxx")
    }
}
