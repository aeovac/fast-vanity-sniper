use std::future::Future;
use tokio::net::TcpStream;
use fastwebsockets::handshake;
use fastwebsockets::{Frame, OpCode};
use hyper::Request;
use hyper_rustls::{ConfigBuilderExt, HttpsConnectorBuilder};
use rustls::ClientConfig;

const TOKEN: &str = "";
const GATEWAY_URL: &str = "";


#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("gateway.discord.gg:443").await;

    let config = ClientConfig::builder()
        .with_native_roots();

    let https = HttpsConnectorBuilder::new()
        .with_tls_config(config)
        .https_only()
        .enable_http2()
        .build();

    let req = Request::builder()
        .uri(GATEWAY_URL)
        .header("Upgrade", "websocket")
        .header("Connection", "Upgrade");


    let (ws, _) = handshake::client(&SpawnExecutor, req, https).await?;
    
    ws.set_writev(false);

    ws.write_frame(Frame::binary(payload));

    while let Some(frame) = ws.read_frame().await?.mask {
        match frame.opcode {
            OpCode::Binary => {}
        }
        
    }
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