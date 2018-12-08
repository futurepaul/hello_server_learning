#[macro_use]
extern crate failure;

use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use std::thread;
use std::time::Duration;

use hello::ThreadPool;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  let pool = ThreadPool::new(4);

  for stream in listener.incoming() {
    let stream = stream.unwrap();
    pool.execute(|| {
        handle_connection(stream);
      })
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 512];
  let how_much_we_read = stream.read(&mut buffer).unwrap();
  // stream.read_exact(&mut buffer).unwrap();

  let get = b"GET / HTTP/1.1";
  let fav = b"GET /favicon.ico HTTP/1.1";
  let sleep = b"GET /sleep HTTP/1.1\r\n";

  let (status_line, filename) = if buffer.starts_with(get) {
    println!("get root");
    ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
  } else if buffer.starts_with(fav) {
    println!("get favicon");
    ("HTTP/1.1 200 OK\r\n\r\n", "favicon.ico")
  } else if buffer.starts_with(sleep) {
    println!("get sleep");
    thread::sleep(Duration::from_secs(5));
    println!("finally done sleeping");
    ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
  } else {
    println!("404");
    ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
  };

  // let contents = fs::read_to_string(filename).unwrap();

  // let response = format!("{}{}", status_line, contents);

  let body = fs::read(filename).unwrap();

  let response = [status_line.as_bytes(), body.as_slice()].concat();

  let how_much_written = stream.write(response.as_slice()).unwrap();
  stream.flush().unwrap();
}
