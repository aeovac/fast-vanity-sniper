use std::future::Future;
use fastwebsockets::{OpCode, Frame};
use fastwebsockets::handshake::client;
use hyper::Request;
use hyper_rustls::{ConfigBuilderExt, HttpsConnectorBuilder};
use rustls::ClientConfig;
use tokio::io::Empty;
use tokio::net::TcpStream;
use bytes::Bytes;
use anyhow::{Result, Context};

#[tokio::main]
async fn main() -> Result<()> {
    const TOKEN: &str = "token";
    const GATEWAY_URL: &str = "wss://gateway.discord.gg/";

    let stream = TcpStream::connect("gateway.discord.gg:443").await?;

    let builder = ClientConfig::builder()
        .with_native_roots()
        .context("")?;

    let config = builder.with_no_client_auth();

    let https = HttpsConnectorBuilder::new()
        .with_tls_config(config)
        .https_only()
        .enable_http1()
        .build();

    let req = Request::builder()
        .uri(GATEWAY_URL)
        .header("Host", "gateway.discord.gg")
        .header("Upgrade", "websocket")
        .header("Connection", "Upgrade")
        .header("Sec-WebSocket-Version", "13")
        .header("Sec-WebSocket-Key", fastwebsockets::handshake::generate_key())
        .body(())?;

    let (ws, _) = client(&SpawnExecutor, req, stream).await?;
    ws.set_writev(false);

    //ws.write_frame(Frame::binary(payload)).await

    loop {
        let frame = ws.read_frame().await
            .context("")?;
    }

    Ok(())
}

struct SpawnExecutor;
impl<Fut> hyper::rt::Executor<Fut> for SpawnExecutor
where
    Fut: Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    fn execute(&self, fut: Fut) {
        tokio::task::spawn(fut);
    }
}
