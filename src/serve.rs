use crate::client::Client;
use crate::parser;
use crate::protocol::StreamStatus;
use crate::DB;
use anyhow::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[derive(Clone)]
pub struct Serve {
    host: String,
    port: u16,
    socket_id: usize,
}

impl Serve {
    pub fn new(host: String, port: u16) -> Self {
        Self {
            host,
            port,
            socket_id: 0,
        }
    }

    // 生成连接ID
    pub fn incr_socket_id(&mut self) -> usize {
        self.socket_id += 1;
        self.socket_id
    }

    pub async fn start(mut self, db: DB) -> Result<(), Error> {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).await?;

        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    // 生成一个新的ID
                    let socket_id = self.incr_socket_id();
                    let db = db.clone();

                    // 处理连接信息
                    tokio::spawn(self.clone().handle_connect(stream, socket_id, db));
                }
                Err(e) => {
                    println!("accept.error listener {:?} error {:?}", listener, e);
                }
            }
        }
    }

    // 处理连接
    pub async fn handle_connect(self, stream: TcpStream, socket_id: usize, db: DB) {
        let mut client = Client::new(socket_id, db);
        println!("client new connect socket.id {}\r\n", socket_id);

        let (mut socket_reader, socket_writer) = stream.into_split();

        // 处理通道消息
        tokio::spawn(client.clone().rev_forward_message(socket_writer));

        loop {
            // 会分批读取
            match socket_reader.read_buf(&mut client.buffer).await {
                Ok(0) => {
                    client.sender.send(Err(StreamStatus::Offline));
                    break;
                }
                Ok(_) => {
                    // 获取到完整的数据包
                    // println!("rev buffer {:#?}", client.buffer);
                    if let Ok(params) = client.get_complete_package() {
                        // 发送数据
                        if let Err(e) = client.sender.send(parser::entry(params)) {
                            println!("client.sender.error {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("read_buf.error {:?}", e);
                    break;
                }
            }
        }

        println!("client {} is closed", socket_id);
    }
}
