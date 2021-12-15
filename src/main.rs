use anyhow::Error;

mod client;
mod protocol;
mod serve;

#[tokio::main]
async fn main() -> Result<(), Error> {
    serve::Serve::new("0.0.0.0".to_string(), 6379).start().await
}
