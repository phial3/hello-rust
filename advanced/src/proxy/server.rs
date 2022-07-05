use crate::mysql::{errcode, packet, utils};
use crate::proxy::errors::{ProxyError, ProxyResult};
use crate::{frontend, router};
use std::error::Error;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};

#[derive(Debug)]
pub struct ProxyServer {}

impl ProxyServer {
    pub fn new() -> ProxyServer {
        ProxyServer {}
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        log::info!("Run sharding proxy server...");
        let shard_r = router::build_router()?;
        log::info!("Shard router module init ok! {:#?}", &shard_r);
        let listen_address = crate::GLOBAL_CONFIG.query_proxy_listen_addr();
        let listener = TcpListener::bind(listen_address).await?;
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let client_router = shard_r.clone();
                    tokio::spawn(async move {
                        if let Err(e) = process(stream, utils::generate_id(), client_router).await {
                            println!("Fail to process connection; error = {}", e);
                        }
                    });
                }
                Err(e) => println!("Accepting socket stream error; error = {:?}", e),
            }
        }
    }
}

async fn process<'a>(
    stream: TcpStream,
    id: u32,
    router: Arc<router::Router<'a>>,
) -> ProxyResult<()> {
    log::info!(
        "Server listener: {}, Accepted from: {}, MySQL thread id: {}",
        stream.local_addr()?,
        stream.peer_addr()?,
        id
    );

    let mut c2p = frontend::conn::C2PConn::build_c2p_conn(stream, id, router).await?;
    if let Err(e) = c2p.s2c_handshake().await {
        let err_p = packet::ErrPacket::new(errcode::ER_HANDSHAKE_ERROR, format!("{:?}", e));
        return c2p
            .write_err(err_p)
            .await
            .map_err(|e| ProxyError::Other(Box::new(e)));
    }
    c2p.run_loop().await;
    Ok(())
}
