use crate::protocol::{KeyType, Protocol};

// hset key field value
pub fn transform(param: Vec<String>) -> Protocol {
    if param.len() != 4 {
        return Protocol::Error("ERR wrong number of arguments for 'hset' command".to_string());
    }

    Protocol::HSet {
        typ: KeyType::Hash,
        key: param[1].to_string(),
        field: param[2].to_string(),
        value: param[3].to_string(),
    }
}
