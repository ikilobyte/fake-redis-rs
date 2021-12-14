use crate::protocol::Protocol;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::{Mutex, MutexGuard};

type Sender = UnboundedSender<Protocol>;

#[derive(Debug, Clone)]
pub struct Client {
    inner: Arc<Mutex<Inner>>,
    pub sender: UnboundedSender<Protocol>,
}

#[derive(Debug)]
pub struct Inner {
    pub id: usize,
    pub reader: UnboundedReceiver<Protocol>,
}

impl Client {
    pub fn new(id: usize) -> Self {
        // 用于内部转发逻辑处理
        let (sender, reader) = tokio::sync::mpsc::unbounded_channel();

        Self {
            inner: Arc::new(Mutex::new(Inner { id, reader })),
            sender,
        }
    }

    // 获取inner
    pub async fn inner(&self) -> MutexGuard<'_, Inner> {
        self.inner.lock().await
    }

    // 处理socket的消息
    pub async fn rev_socket_message(self, socket_reader: OwnedReadHalf) {
        println!("{:#?}", socket_reader);
    }

    // 接收通道的消息
    pub async fn rev_forward_message(self, mut socket_writer: OwnedWriteHalf) {
        let mut inner = self.inner().await;
        while let Some(protocol) = inner.reader.recv().await {
            match protocol {
                Protocol::Command => {
                    socket_writer.write(b"+OK\r\n").await;
                }
                Protocol::Set {
                    key,
                    value,
                    ttl,
                    lock,
                } => {
                    println!("{:#?}", "set cmd");
                }
                Protocol::Get { .. } => {}
                Protocol::UnSupport => {}
                Protocol::Error(mut e) => {
                    let err = format!("-{}\r\n", e);
                    socket_writer.write(err.as_bytes()).await;
                }
            }
        }
    }
}
