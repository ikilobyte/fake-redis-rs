use crate::client::Client;
use crate::protocol::Parser;
use crate::DB;
use anyhow::Error;
use bytes::{Bytes, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[derive(Clone, Debug)]
pub struct Serve {
    host: String,
    port: u16,
    socket_id: usize,
    buffer: Vec<u8>,
    cursor: usize,
}

impl Serve {
    pub fn new(host: String, port: u16) -> Self {
        Self {
            host,
            port,
            socket_id: 0,
            buffer: vec![0; 100],
            cursor: 0,
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
    pub async fn handle_connect(mut self, stream: TcpStream, socket_id: usize, db: DB) {
        let client = Client::new(socket_id, db);
        println!("client new connect socket.id {}\r\n", socket_id);

        let (mut socket_reader, mut socket_writer) = stream.into_split();

        // 处理通道消息
        // tokio::spawn(client.clone().rev_forward_message(socket_writer));

        let mut count = 1;

        // 获取到第一个*，然后开始解析这一行的数据
        let mut lines: Vec<String> = vec![];
        let bytes: Vec<&str> = Vec::new();

        loop {
            // 使用Bytes可以读取一个完整的数据
            println!("第{}次读取! start", count);
            match socket_reader.read(&mut self.buffer).await {
                Ok(0) => {
                    println!("{:?}", self.buffer);
                    break;
                }
                Ok(_) => {
                    socket_writer.write(b"+OK\r\n").await;
                }
                Err(e) => {
                    println!("read.error {:?}", e);
                    break;
                }
            }

            println!("第{}次读取! end", count);

            // 读取一个，检测一个，

            count += 1;
            // if let Ok(_) = socket_reader.read(&mut self.buffer).await {
            //     // 退出了
            //
            //     println!("{:#?}", self.buffer);
            //     socket_writer.write(b"+OK\r\n").await;
            //     if self.buffer.is_empty() {
            //         break;
            //     }
            //
            //     // let content = String::from_utf8_lossy(&buffer.to_vec()[..]).to_string();
            //     //
            //     // let parse = Parser::start(content);
            //     //
            //     // println!("{:#?}", parse);
            //     // // 转发出去
            //     // if let Err(e) = client.sender.send(parse) {
            //     //     println!("client.sender.send error {:?}", e);
            //     //     continue;
            //     // }
            // }
        }

        println!("client {} is closed", socket_id);
    }

    pub fn check_frame(&self) {}
}
