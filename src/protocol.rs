use crate::config::PARSE_LIST;
use crate::storage::types::{KeyType, Message};
use std::any::Any;
use std::collections::HashMap;

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

// vec转为命令
impl From<Vec<String>> for Protocol {
    fn from(params: Vec<String>) -> Self {
        // 参数错误了
        if params.len() < 2 {
            return Protocol::UnSupport;
        }

        // 处理一下，只保留实用的数据
        let mut param_filter = vec![];
        for (index, param) in params[2..].to_vec().iter().enumerate() {
            if index % 2 == 0 {
                param_filter.push(param.clone());
            }
        }

        let cmd = &param_filter[0].to_uppercase()[..];

        println!("解析cmd {}", cmd);
        // 执行解析
        if let Some(parser) = PARSE_LIST.get(cmd) {
            return parser(param_filter);
        }

        return Protocol::UnSupport;
    }
}
