use crate::protocol::Protocol;
use crate::storage::types::KeyType;

// hget key field
pub fn transform(param: Vec<String>) -> Protocol {
    if param.len() != 3 {
        return Protocol::Error("ERR wrong number of arguments for 'hget' command".to_string());
    }

    Protocol::HGet {
        typ: KeyType::Hash,
        key: param[1].to_string(),
        field: param[2].to_string(),
    }
}
