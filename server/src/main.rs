use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use server::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let request_parts: Vec<&str> = request_line.split_whitespace().collect();
    let path = if request_parts.len() >= 2 {
        request_parts[1]
    } else {
        "/"
    };

    let file_path = match path {
        "/" => "../public/index.html".to_string(),
        _ => format!("../public{}", path),
    };

    let (status_line, content_type, contents) = match fs::read(&file_path) {
        Ok(contents) => {
            let content_type = get_content_type(&file_path);
            ("HTTP/1.1 200 OK", content_type, contents)
        }
        Err(_) => {
            let fallback =
                fs::read("../public/404.html").unwrap_or_else(|_| b"<h1>404</h1>".to_vec());
            ("HTTP/1.1 404 NOT FOUND", "text/html", fallback)
        }
    };

    let response_header = format!(
        "{status_line}\r\nContent-Length: {}\r\nContent-Type: {content_type}\r\n\r\n",
        contents.len()
    );

    stream.write_all(response_header.as_bytes()).unwrap();
    stream.write_all(&contents).unwrap();
}

fn get_content_type(path: &str) -> &str {
    if path.ends_with(".html") {
        "text/html"
    } else if path.ends_with(".css") {
        "text/css"
    } else if path.ends_with(".js") {
        "application/javascript"
    } else if path.ends_with(".mp4") {
        "video/mp4"
    } else {
        "application/octet-stream"
    }
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
