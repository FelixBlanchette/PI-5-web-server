use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use server::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    // Update the file paths to reflect the new location of the 'public' directory
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "~/public/index.html") // file is in the root 'public' folder
    } else {
        ("HTTP/1.1 404 NOT FOUND", "~/public/404.html") // file is in the root 'public' folder
    };

    // Attempt to read the requested file
    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => {
            // If the file is not found, return a 404 response
            let not_found_message = "<h1>404 - Page Not Found </h1>";
            return send_response(&mut stream, "HTTP/1.1 404 NOT FOUND", not_found_message);
        }
    };

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn send_response(stream: &mut TcpStream, status_line: &str, body: &str) {
    let response = format!(
        "{status_line}\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for streams in listener.incoming() {
        let s = streams.unwrap();
        pool.execute(|| {
            handle_connection(s);
        });
    }
}
