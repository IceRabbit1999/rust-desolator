use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kv_server::{CommandRequest, CommandResponse};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()>{
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening on: {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client {:?} connected", addr);
        tokio::spawn(async move {
            let mut stream = AsyncProstStream::<_, CommandRequest, CommandResponse, _>::from(stream).for_async();

        })
    }

    Ok(())
}