use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_connection(stream);
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 512];
  let how_much_we_read = stream.read(&mut buffer).unwrap();

  let get = b"GET / HTTP/1.1";
  let fav = b"GET /favicon.ico HTTP/1.1";
  // "GET /favicon.ico HTTP/1.1" 404 183

  let (status_line, filename) = if buffer.starts_with(get) {
    println!("get root");
    ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
  } else if buffer.starts_with(fav) {
    println!("get favicon");
    ("HTTP/1.1 200 OK\r\n\r\n", "favicon.ico")
  } else {
    println!("404");
    ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
  };

  let contents = fs::read_to_string(filename).unwrap();

  let response = format!("{}{}", status_line, contents);
  let how_much_written = stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}
