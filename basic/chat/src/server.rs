use std::error::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

const LOCAL_SERVER: &str = "127.0.0.1:8888";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(LOCAL_SERVER).await?;
    let (tx, _rx) = broadcast::channel(12);
    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("{} connected", addr);
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut msg = String::new();
            loop {
                // let bytes_read = reader.read_line(&mut msg).await.unwrap();
                // if bytes_read == 0 {
                //     break;
                // }
                // println!("{}", msg);
                // writer.write_all(msg.as_bytes()).await.unwrap();
                // msg.clear();

                tokio::select! {
                    result = reader.read_line(&mut msg) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        println!("{}", msg);
                        tx.send((msg.clone(), addr)).unwrap();
                        msg.clear();
                    }
                    result = rx.recv() => {
                        let (msg_str, other_addr) = result.unwrap();
                        if addr != other_addr {
                            println!("send {} to {}", &msg_str, other_addr);
                            writer.write_all(msg_str.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
    // OK(())
}
