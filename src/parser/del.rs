use crate::protocol::{KeyType, Protocol};

// del key [key ...]
pub fn transform(param: Vec<String>) -> Protocol {
    if param.len() < 2 {
        return Protocol::Error("ERR wrong number of arguments for 'del' command".to_string());
    }

    Protocol::Del {
        typ: KeyType::String,
        keys: param[1..].to_vec(),
    }
}
