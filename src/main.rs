use anyhow::Error;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

mod client;
mod protocol;
mod serve;

#[tokio::main]
async fn main() -> Result<(), Error> {
    /**
    b"*1\r\n$7\r\nCOMMAND\r\n"
    收到数据 -> Command
    b"*3\r\n$3\r\nset\r\n$1\r\na\r\n$1\r\n1\r\n"
    收到数据 -> Command
    b"*2\r\n$3\r\nget\r\n$1\r\na\r\n"
    **/
    let mut serve = serve::Serve::new("0.0.0.0".to_string(), 6379);

    Ok(serve.start().await?)
    // let pro = protocol::Parser::start(1);
    // println!("{:#?}", pro);
    // Ok(())
}
