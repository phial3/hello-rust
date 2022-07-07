pub mod server {
    use std::io::prelude::*;
    pub use std::sync::mpsc;
    use std::net::TcpStream;

    pub fn handle_connection(
        mut stream: TcpStream,
        display_data: u32,
        client: mpsc::Sender<u32>)
    {
        let mut buffer = [ 0; 1024 ];
        stream.read(&mut buffer).unwrap();

        let request_data = get_request_data(&buffer);
        client.send(request_data).unwrap();
        send_response(stream, display_data, request_data)
    }

    fn get_request_data(buffer: &[u8; 1024]) -> u32
    {
        let request = String::from_utf8_lossy(buffer);
        let striped_request = request.replace("GET /", "");

        let mut request_data = 1u32;
        if let Some(uri_end) = striped_request.find(" ") {
            let uri = &striped_request[..uri_end];
            if let Ok(parsed) = uri.parse::<u32>() { request_data = parsed }
        }

        request_data
    }

    fn send_response(
        mut stream: TcpStream,
        display_data: u32,
        request_data: u32)
    {
        let page = format!("
        <!DOCTYPE html>
        <html lang=\"en\">
        <head>
            <meta charset=\"utf-8\">
            <title>Hello!</title>
        </head>
        <body>
            <h1>Hello!</h1>
            <p>The current value is {}.</p>
        </body>
        </html>", display_data + request_data);

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            page.len(),
            page
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        println!("{} Response sent.", display_data + request_data);
    }
}
