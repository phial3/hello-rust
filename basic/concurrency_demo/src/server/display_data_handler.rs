pub mod data_handler {
    pub use std::sync::mpsc;

    pub fn handle_display_data(
        server: mpsc::Receiver<u32>,
        display_data_writer: mpsc::Sender<u32>)
    {
        loop {
            let display_data = server.recv().unwrap();
            display_data_writer.send(display_data).unwrap();
        }
    }
}
