use std::net::TcpListener;
use std::sync::mpsc;
use threadpool::ThreadPool;

mod http_server;
pub use crate::http_server::server;

mod display_data_handler;
pub use crate::display_data_handler::data_handler;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(100);
    let (client, server) = mpsc::channel();
    let (display_data_writer, display_data_reader) = mpsc::channel();

    pool.execute(|| {
        data_handler::handle_display_data(server, display_data_writer);
    });

    let mut display_data = 0u32;
    for stream in listener.incoming() {

        if let Ok(new_data) = display_data_reader.try_recv() {
            display_data += new_data;
        }

        println!("Received display_data: {}", display_data);

        let stream = stream.unwrap();
        let client_n = client.clone();
        pool.execute(move || {
            server::handle_connection(stream, display_data, client_n);
        });
    }
}
