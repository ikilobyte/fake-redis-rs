use crate::storage::types::KeyType;

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
