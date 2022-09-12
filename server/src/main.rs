use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};
use gthread::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    while match stream.read(&mut buffer) {
        Ok(size) => {
            stream.write(&buffer[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("Error; Terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on port 7878");
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                pool.execute(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e)
            }
        }
    }
    println!("Shutting down.");
}
