use crate::protocol::Protocol;
use crate::storage::types::Message;
use crate::DB;
use bytes::BytesMut;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::{Mutex, MutexGuard};

#[derive(Debug, Clone)]
pub struct Client {
    inner: Arc<Mutex<Inner>>,
    pub sender: UnboundedSender<Result<Message, ()>>,
    db: DB,
    pub param_number: usize,
    pub cursor: usize,
    pub buffer: BytesMut,
    pub params: Vec<String>,
    head: bool,
}

#[derive(Debug)]
pub struct Inner {
    pub id: usize,
    pub reader: UnboundedReceiver<Result<Message, ()>>,
}

impl Client {
    pub fn new(id: usize, db: DB) -> Self {
        // 用于内部转发逻辑处理
        let (sender, reader) = tokio::sync::mpsc::unbounded_channel();

        Self {
            inner: Arc::new(Mutex::new(Inner { id, reader })),
            sender,
            db,
            param_number: 0,
            cursor: 0,
            buffer: BytesMut::new(),
            params: vec![],
            head: false,
        }
    }

    // 获取inner
    pub async fn inner(&self) -> MutexGuard<'_, Inner> {
        self.inner.lock().await
    }

    // 接收通道的消息
    pub async fn rev_forward_message(self, mut socket_writer: OwnedWriteHalf) {
        let mut inner = self.inner().await;
        while let Some(resp) = inner.reader.recv().await {
            if let Err(_) = resp {
                socket_writer.write(b"-ERR syntax error\r\n").await;
                continue;
            }

            let message = resp.unwrap();
            let mm = message.clone();
            let protocol = message.protocol;

            match protocol {
                Protocol::Command => {
                    if let Err(e) = socket_writer.write(b"+OK\r\n").await {
                        println!("client establish connection  write error -> {:#?}", e);
                        continue;
                    }
                }
                Protocol::UnSupport => {
                    if let Err(e) = socket_writer.write(b"-command unsupport\r\n").await {
                        println!("Protocol::UnSupport write error -> {:#?}", e);
                        continue;
                    }
                }
                Protocol::Error(e) => {
                    let err = format!("-{}\r\n", e);
                    if let Err(e) = socket_writer.write(err.as_bytes()).await {
                        println!("Protocol::Error write error -> {:#?}", e);
                        continue;
                    }
                }

                // 可以执行的类型
                _ => {
                    // TODO 设置的类型是否符合之前的类型
                    // TODO 之前是 string，在未被删除前，都是string类型，不能改变类型
                    let resp = if let Ok(resp) = self.db.handle(mm).await {
                        resp
                    } else {
                        "-Internal Server Error\r\n".to_string()
                    };

                    // 写数据错误应该是连接都断开了
                    if let Err(x) = socket_writer.write(resp.as_bytes()).await {
                        println!("socket write error {:?}", x);
                        break;
                    }
                }
            }
        }
    }

    // 获取完整的数据包，协议规范：https://redis.io/topics/protocol
    // 如：*1\r\n$7\r\nCOMMAND\r\n
    // 如：*3\r\n$3\r\nset\r\n$3\r\nkey\r\n$5\r\nvalue\r\n
    // 如：*4\r\n$4\r\nhset\r\n$3\r\nkey\r\n$5\r\nfield\r\n$5\r\nvalue\r\n
    // 数据过长会分批读取到self.buffer中，所以需要分批解析出数据包，解析完毕后，恢复初始状态
    pub fn get_complete_package(&mut self) -> Result<Vec<String>, ()> {
        for mut i in self.cursor..self.buffer.len() {
            // 一行的开始
            let byte = self.buffer[i];
            if byte == b'*' {
                self.head = true;
            }

            // 找出这个包一共有几个参数
            if self.head && self.buffer[i] == b'\r' && self.buffer[i + 1] == b'\n' {
                let content = &String::from_utf8_lossy(&self.buffer[self.cursor..i]).to_string();
                self.param_number = content[1..].parse().unwrap();
                self.head = false;
                self.params.push(content.clone());
                self.cursor += i + 2;
                i += 2;
            }

            // 开始寻找所有参数
            if self.param_number >= 1 {
                if self.buffer[i] == b'\r' && self.buffer[i + 1] == b'\n' {
                    // 改变参数和游标
                    self.params
                        .push(String::from_utf8_lossy(&self.buffer[self.cursor..i]).to_string());
                    self.cursor = i + 2;
                }

                // 参数完整
                if self.param_number * 2 + 1 == self.params.len() {
                    // 重置状态，等待下一个完整的数据包
                    let params = self.params.clone();
                    self.head = false;
                    self.param_number = 0;
                    self.cursor = 0;
                    self.buffer = BytesMut::new();
                    self.params = vec![];

                    return Ok(params);
                }
            }
        }

        return Err(());
    }
}
