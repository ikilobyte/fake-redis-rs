use crate::protocol::Protocol;
use crate::storage::hash::THash;
use crate::storage::string::TString;
use crate::storage::types::KeyType;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

// 这里存储的运行时保存的数据
// 目前没有做持久化，进程结束后，数据消失
#[derive(Debug)]
pub struct DB {
    pub inner: Arc<Mutex<Inner>>,
}

#[derive(Debug)]
pub struct Inner {
    pub keys: HashMap<String, KeyType>, // 保存所有的key
    pub t_string: TString,
    pub t_hash: THash,
}

impl DB {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Inner {
                keys: HashMap::new(),
                t_string: TString::new(),
                t_hash: THash::new(),
            })),
        }
    }

    pub async fn inner(&self) -> MutexGuard<'_, Inner> {
        self.inner.lock().await
    }

    // 处理传过来的数据
    pub async fn handle(&self, message: Protocol) -> Result<String, ()> {
        let mut inner = self.inner().await;

        let mm = message.clone();
        let resp = match message {
            Protocol::Set {
                cmd: _cmd,
                typ: _typ,
                key,
                value,
                ttl,
                lock,
            } => inner.t_string.set(key, value, ttl, lock),

            Protocol::Get { cmd, key } => inner.t_string.get(key),
            Protocol::HSet {
                cmd,
                key,
                field,
                value,
            } => inner.t_hash.set(key, field, value),
            _ => Ok("+OK\r\n".to_string()),
        };

        // 操作成功时，保存所有的key，和key对应的类型
        if let Ok(_) = resp {
            let key_type: KeyType = mm.into();
            println!("key_type {:?}", key_type);
        }

        resp
    }
}

impl Clone for DB {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl Clone for Inner {
    fn clone(&self) -> Self {
        Self {
            keys: self.keys.clone(),
            t_hash: self.t_hash.clone(),
            t_string: self.t_string.clone(),
        }
    }
}
