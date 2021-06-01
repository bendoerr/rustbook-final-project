use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(l) => l,
        Err(err) => {
            eprintln!("Failed to bind port: {}", err);
            return ();
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(err) => {
                eprintln!("Stream error: {}", err);
            }
        };
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("Connection established!");

    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(read) => {
            println!("Read {} bytes:\n{}", read, String::from_utf8_lossy(&buffer[..]).split("\n").map(|l| format!(" < {}\n", l)).collect::<String>());
        }
        Err(err) => {
            eprintln!("Read error: {}", err);
            return;
        }
    };

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let response = match fs::read_to_string(filename) {
        Ok(contents) => {
            format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                status_line,
                contents.len(),
                contents
            )
        }
        Err(err) => {
            eprintln!("Get contents failed: {}", err);
            return;
        }
    };

    match stream.write(response.as_bytes()) {
        Ok(wrote) => {
            println!("Wrote {} bytes", wrote);
        }
        Err(err) => {
            eprintln!("Failed to write: {}", err);
            return;
        }
    }

    match stream.flush() {
        Ok(()) => println!("Flushed"),
        Err(err) => {
            eprintln!("Failed to flush: {}", err);
            return;
        }
    }

    println!("Connection handled!");
}
