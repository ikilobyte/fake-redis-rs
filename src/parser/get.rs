use crate::protocol::{KeyType, Protocol};

// 解析：get key
pub fn transform(param: Vec<String>) -> Protocol {
    // 语法错误
    if param.len() != 2 {
        return Protocol::Error("ERR syntax error".to_string());
    }
    Protocol::Get {
        typ: KeyType::String,
        key: param[1].to_string(),
    }
}
