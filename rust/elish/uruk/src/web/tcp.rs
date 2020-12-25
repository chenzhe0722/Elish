use async_std::net::{TcpListener, TcpStream, ToSocketAddrs};
use async_std::prelude::*;
use async_std::task::spawn;
use log::{error, info, warn};
use std::io::Result;

#[inline]
pub async fn serve<F>(addrs: impl ToSocketAddrs, handler: fn(TcpStream) -> F) -> Result<()>
where
    F: Future<Output = Result<()>> + Send + 'static,
{
    match TcpListener::bind(addrs).await {
        Ok(listener) => Ok(accept(listener, handler).await),
        Err(err) => Err(err),
    }
}

#[inline]
async fn accept<F>(listener: TcpListener, handler: fn(TcpStream) -> F)
where
    F: Future<Output = Result<()>> + Send + 'static,
{
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        match stream {
            Ok(stream) => {
                match stream.peer_addr() {
                    Ok(addr) => info!("Accept incoming from: {}", addr),
                    Err(err) => warn!("Accept incoming from unknown address. At: {}", err),
                }
                handle(stream, handler);
            }
            Err(err) => error!("Accept incoming error. At: {}", err),
        }
    }
}

#[inline]
fn handle<F>(stream: TcpStream, handler: fn(TcpStream) -> F)
where
    F: Future<Output = Result<()>> + Send + 'static,
{
    spawn(async move {
        if let Err(err) = spawn(handler(stream)).await {
            error!("Handle stream error. At: {}", err)
        }
    });
}
