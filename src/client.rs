use crate::protocol::Protocol;
use crate::storage::types::Message;
use crate::DB;
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
}
