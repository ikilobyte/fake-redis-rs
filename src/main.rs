use crate::storage::db::DB;
use anyhow::Error;
use std::io::Cursor;
use tokio::fs::File;
use tokio::io;

mod client;
mod protocol;
mod serve;
mod storage;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let mut reader: &[u8] = b"hello";
    // let mut file = File::create("foo.txt").await?;
    // io::copy(&mut reader, &mut file).await?;

    // 完整的包数据
    // let full = String::from_utf8_lossy(b"*3\r\n$3\r\nset\r\n$3\r\nkey\r\n$5\r\nvalue\r\n");

    // *3\r\n$3\r\nset\r\n$3\r\nkey\r\n$5\r\nvalue\r\n
    let mut cursor = Cursor::new(b"*3\r\n$3\r\nset\r\n$3\r\nkey\r\n$5\r\nvalue\r\n");
    let mut list = vec![];
    for i in 0..cursor.get_ref().len() {
        if cursor.get_ref()[i] == b'\r' && cursor.get_ref()[i + 1] == b'\n' {
            let position = cursor.position() as usize;
            let ref line = cursor.get_ref()[position..i];
            list.push(String::from_utf8_lossy(line).to_string());
            cursor.set_position((i + 2) as u64);
        }
    }
    println!("{:#?}", list);
    let size = list[0][1..].parse::<usize>().unwrap();

    println!("{:#?}", &list[2..]);
    let i = &list[2..];
    let mut params = vec![];
    for (i, item) in i.iter().enumerate() {
        if i % 2 == 0 {
            params.push(item);
        }
    }

    println!("{:#?} {:?} ", params.len(), params);
    if params.len() == size {
        println!("{:#?}", "这是一个完整的数据!");
    }
    let db = DB::new();
    serve::Serve::new("0.0.0.0".to_string(), 6379)
        .start(db)
        .await
}
