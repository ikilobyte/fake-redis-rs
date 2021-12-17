use crate::storage::db::DB;
use anyhow::Error;

mod client;
mod config;
mod parser;
mod protocol;
mod serve;
mod storage;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let db = DB::new();
    serve::Serve::new("0.0.0.0".to_string(), 6379)
        .start(db)
        .await
}
