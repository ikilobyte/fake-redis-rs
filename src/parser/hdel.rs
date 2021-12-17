use crate::protocol::Protocol;
use crate::storage::types::KeyType;

// hdel key field [field ...]
pub fn transform(param: Vec<String>) -> Protocol {
    if param.len() < 3 {
        return Protocol::Error("ERR wrong number of arguments for 'hdel' command".to_string());
    }

    Protocol::HDel {
        typ: KeyType::Hash,
        key: param[1].clone(),
        fields: param[2..].to_vec(),
    }
}
