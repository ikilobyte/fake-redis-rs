use crate::protocol::Protocol;
use lazy_static::lazy_static;
use std::collections::HashMap;
//
// lazy_static! {
//     pub static ref CMD_REF_TYPE: HashMap<String, KeyType> = {
//         let mut map = HashMap::new();
//         map.insert("SET".to_string(), KeyType::String);
//         map.insert("GET".to_string(), KeyType::String);
//
//         // hash相关的命令
//         map.insert("HSET".to_string(), KeyType::Hash);
//         map.insert("HGET".to_string(), KeyType::Hash);
//         map.insert("HDEL".to_string(), KeyType::Hash);
//         map.insert("HLEN".to_string(), KeyType::Hash);
//         map.insert("HKEYS".to_string(), KeyType::Hash);
//         map.insert("HVALS".to_string(), KeyType::Hash);
//
//         map
//     };
// }

// redis的五种数据类型
#[derive(Clone, Debug, PartialEq)]
pub enum KeyType {
    String,
    Hash,
    List,
    Set,
    ZSet,
}

impl From<Protocol> for KeyType {
    fn from(msg: Protocol) -> Self {
        match msg {
            Protocol::Set { typ, .. } => typ,
            Protocol::Get { typ, .. } => typ,
            Protocol::HSet { typ, .. } => typ,
            Protocol::HGet { typ, .. } => typ,

            // 根本不会执行到这里，所以这个panic也根本就不会执行！
            _ => panic!(""),
        }
    }
}
