use crate::client::Client;
use crate::protocol::Parser;
use crate::DB;
use anyhow::Error;
use bytes::BytesMut;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

#[derive(Clone, Debug)]
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
            if let Ok((stream, _)) = listener.accept().await {
                // 生成一个新的ID
                let socket_id = self.incr_socket_id();
                let db = db.clone();

                // 处理连接信息
                tokio::spawn(self.clone().handle_connect(stream, socket_id, db));
            }
        }

        // Ok(())
    }

    // 处理连接
    pub async fn handle_connect(self, stream: TcpStream, socket_id: usize, db: DB) {
        let client = Client::new(socket_id, db);
        println!("client new connect socket.id {}\r\n", socket_id);

        let (mut socket_reader, socket_writer) = stream.into_split();

        // 处理通道消息
        tokio::spawn(client.clone().rev_forward_message(socket_writer));

        loop {
            // 使用Bytes可以读取一个完整的数据
            // TODO 数据过长会有问题
            let mut buffer = BytesMut::with_capacity(1024 * 10);
            if let Ok(_) = socket_reader.read_buf(&mut buffer).await {
                // 退出了
                if buffer.is_empty() {
                    break;
                }

                let content = String::from_utf8_lossy(&buffer.to_vec()[..]).to_string();

                let parse = Parser::start(content);

                println!("{:#?}", parse);
                // 转发出去
                if let Err(e) = client.sender.send(parse) {
                    println!("client.sender.send error {:?}", e);
                    continue;
                }
            }
        }

        println!("client {} is closed", socket_id);
    }
}
