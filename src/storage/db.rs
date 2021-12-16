use crate::protocol::Protocol;
use crate::storage::hash::THash;
use crate::storage::string::TString;
use crate::storage::types::{KeyType, Message, WRITE_CMD};
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
    pub async fn handle(&self, message: Message) -> Result<String, ()> {
        let mut inner = self.inner().await;

        let key = message.key.clone();
        let protocol = message.protocol.clone();
        let key_type = protocol.clone().into();

        // 判断cmd
        if !self.check_typ_unanimous(&inner, &key, &key_type) {
            return Ok(
                "-WRONGTYPE Operation against a key holding the wrong kind of value\r\n"
                    .to_string(),
            );
        }

        let resp = match protocol {
            Protocol::Set {
                typ,
                key,
                value,
                ttl,
                lock,
            } => inner.t_string.set(key, value, ttl, lock),

            Protocol::Get { typ, key } => inner.t_string.get(key),
            Protocol::HSet {
                typ,
                key,
                field,
                value,
            } => inner.t_hash.set(key, field, value),
            Protocol::HGet { typ, key, field } => inner.t_hash.get(key, field),
            _ => Ok("+OK\r\n".to_string()),
        };

        // 只有是"写"类型的命令才能保存
        if WRITE_CMD.contains(&&message.cmd[..]) {
            if let Ok(_) = resp {
                inner.keys.insert(key, key_type);
            }
        }

        println!("{:#?}", inner.keys);
        resp
    }

    // 检测类型是否一致
    fn check_typ_unanimous(
        &self,
        inner: &MutexGuard<'_, Inner>,
        key: &String,
        cmd_typ: &KeyType,
    ) -> bool {
        //todo 删除类型的命令不需要判断！
        if let Some(real_typ) = inner.keys.get(key) {
            if real_typ == cmd_typ {
                return true;
            }
        } else {
            return true;
        }

        false
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
