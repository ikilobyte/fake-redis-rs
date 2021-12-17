use crate::config::WRITE_CMD;
use crate::protocol::{KeyType, Message, Protocol};
use crate::storage::hash::THash;
use crate::storage::string::TString;
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

        println!("inner {:?} self.inner {:?}", inner, self.inner);
        let key = message.key.clone();
        let protocol = message.protocol.clone();
        let key_type = protocol.clone().into();

        // 判断cmd
        if !self.check_typ_unanimous(&inner, &message.cmd, &key, &key_type) {
            return Ok(
                "-WRONGTYPE Operation against a key holding the wrong kind of value\r\n"
                    .to_string(),
            );
        }

        let resp = match protocol {
            Protocol::Set {
                typ: _,
                key,
                value,
                ttl,
                lock,
            } => inner.t_string.set(key, value, ttl, lock),

            Protocol::Get { typ: _, key } => inner.t_string.get(key),
            Protocol::HSet {
                typ: _,
                key,
                field,
                value,
            } => inner.t_hash.set(key, field, value),
            Protocol::HGet { typ: _, key, field } => inner.t_hash.get(key, field),
            Protocol::HDel {
                typ: _,
                key,
                fields,
            } => {
                let resp = inner.t_hash.del(key.clone(), fields);

                // field中的数据全部被删除，那就需要删除keys中的数据
                if let None = inner.t_hash.inner.get(&key[..]) {
                    self.remove(&mut inner, &key[..]);
                }
                resp
            }
            Protocol::Del { typ: _, keys } => self.del(&mut inner, keys),
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

    // https://redis.io/commands/del
    fn del(&self, inner: &mut MutexGuard<'_, Inner>, keys: Vec<String>) -> Result<String, ()> {
        let mut count = 0;

        if inner.keys.len() <= 0 {
            return Ok(format!(":{}\r\n", count));
        }

        for key in keys.iter() {
            if let Some(key_type) = inner.keys.get(key) {
                match key_type {
                    KeyType::String => {
                        // string内部删除
                        if inner.t_string.remove(&key[..]) && self.remove(inner, &key[..]) {
                            count += 1;
                        }
                    }
                    KeyType::Hash => {
                        // hash内部删除
                        if inner.t_hash.remove(&key[..]) && self.remove(inner, &key[..]) {
                            count += 1;
                        }
                    }
                    KeyType::List => {}
                    KeyType::Set => {}
                    KeyType::ZSet => {}
                };
            }
        }

        Ok(format!(":{}\r\n", count))
    }

    // 检测类型是否一致
    fn check_typ_unanimous(
        &self,
        inner: &MutexGuard<'_, Inner>,
        cmd: &String,
        key: &String,
        cmd_typ: &KeyType,
    ) -> bool {
        // 危险命令
        if ["DEL", "FLUSHALL", "FLUSHDB"].contains(&&cmd[..]) {
            return true;
        }

        if let Some(real_typ) = inner.keys.get(key) {
            if real_typ == cmd_typ {
                return true;
            }
        } else {
            return true;
        }

        false
    }

    // 内部删除
    fn remove(&self, inner: &mut MutexGuard<'_, Inner>, key: &str) -> bool {
        if let Some(_) = inner.keys.remove(key) {
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
