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
    bytes: Vec<u8>,
    cursor: usize,
    param_number: usize,
}

impl Serve {
    pub fn new(host: String, port: u16) -> Self {
        Self {
            host,
            port,
            socket_id: 0,
            bytes: Vec::new(),
            cursor: 0,
            param_number: 0,
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

        let mut lines: Vec<String> = Vec::new();

        loop {
            // 使用Bytes可以读取一个完整的数据
            println!("第{}次读取! start", count);
            let mut buffer = vec![0; 6];
            match socket_reader.read(&mut buffer).await {
                Ok(0) => {
                    println!("{:?}", buffer);
                    break;
                }
                Ok(_) => {
                    // 缓存到一个地方去
                    self.bytes.extend(buffer.clone().iter().filter(|item| {
                        if **item > 0 {
                            true
                        } else {
                            false
                        }
                    }));
                    println!("bytes {:?} len {}", self.bytes, self.bytes.len());

                    // 解析出数据
                    for i in 0..self.bytes.len() {
                        if i >= self.bytes.len() - 1 {
                            break;
                        }
                        let byte = self.bytes[i];
                        let next_byte = self.bytes[i + 1];

                        // 解析出一行
                        if byte == b'\r' && next_byte == b'\n' {
                            println!("cursor {} i {} len {}", self.cursor, i, self.bytes.len());

                            // 这里有bug
                            let ref param = self.bytes[self.cursor..i];

                            // 记录有几个参数
                            if param[0] == b'*' {
                                self.param_number = String::from_utf8_lossy(&param[1..])
                                    .parse::<usize>()
                                    .unwrap();
                                self.cursor = i + 2;
                            } else {
                                let str = String::from_utf8_lossy(&param).to_string();
                                lines.push(str);
                                self.cursor = i + 2;
                            }

                            if lines.len() == self.param_number * 2 {
                                self.cursor = 0;
                                println!("读取到了完整的数据");
                                println!("{:#?}", lines);
                                break;
                            }

                            println!(
                                "i+2 {} cursor {} len {}",
                                i + 2,
                                self.cursor,
                                self.bytes.len()
                            );
                        }
                    }

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
