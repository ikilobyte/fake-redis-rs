// use crate::storage::types::KeyType;

#[derive(Debug, Clone)]
pub enum Protocol {
    // 暂时不做什么处理，只用来处理连接
    Command,

    // set key value [EX seconds] [PX milliseconds] [NX|XX]
    Set {
        typ: KeyType,
        key: String,               // key
        value: String,             // value
        ttl: Option<(TTL, usize)>, // 过期时间
        lock: Option<Lock>,        // 排斥相关
    },

    // get key
    Get {
        typ: KeyType,
        key: String, // 获取哪个key
    },

    // hset key field value
    HSet {
        typ: KeyType,
        key: String,
        field: String,
        value: String,
    },

    // hget key field
    HGet {
        typ: KeyType,
        key: String,
        field: String,
    },

    // hdel key field [field ...]
    HDel {
        typ: KeyType,
        key: String,
        fields: Vec<String>,
    },

    // del key [key ...]
    Del {
        typ: KeyType,
        keys: Vec<String>,
    },
    UnSupport,
    Error(String),
}

#[derive(Debug, Clone)]
pub enum TTL {
    EX,
    PX,
}

#[derive(Debug, Clone)]
pub enum Lock {
    NX, // 只在键不存在时， 才对键进行设置操作
    XX, // 只在键已经存在时， 才对键进行设置操作。
}

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
            Protocol::HDel { typ, .. } => typ,
            Protocol::Del { typ, .. } => typ,

            // 根本不会执行到这里，所以这个panic也根本就不会执行！
            _ => panic!(""),
        }
    }
}

// 通道中传递的是这个数据
#[derive(Debug, Clone)]
pub struct Message {
    pub protocol: Protocol,
    pub key: String,
    pub cmd: String,
}

#[derive(Debug)]
pub enum StreamStatus {
    Online,
    Offline,
}
