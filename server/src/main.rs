use std::{
    fs::read_to_string,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};
use gthread::ThreadPool;

fn handle_connection(mut stream: TcpStream, mut msg_vec: Vec<String>) {
    let mut buffer = [0; 1000];
    // reads bytes into buffer 
    stream.read(&mut buffer).unwrap();

    // would be better done as a match
    let get = b"GET /get HTTP/1.1\r\n";
    let post = b"POST /post HTTP/1.1\r\n";
    // request handling
    let response: String = if buffer.starts_with(post) {
        handle_post(&buffer, msg_vec) 
    } else if buffer.starts_with(get) {
        handle_get(msg_vec)
    } else {
         handle_not_found()
    };
    

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_post(mut buffer: &[u8; 1000], mut msg_vec: Vec<String>) -> String {
    
    String::from("HTTP/1.1 200 OK")
}

fn handle_get(mut msg_vec: Vec<String>) -> String {
    
    msg_vec.pop().unwrap()
}

fn handle_not_found() -> String {
        String::from("HTTP/1.1 404 NOT FOUND")
}

fn main() {
    let mut msg_vec: Vec<String> = Vec::new();
    msg_vec.push(String::from("This is the first message"));
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(6) {
        let stream = stream.unwrap();
        let mut msg_vec_clone = msg_vec.clone();
        pool.execute(|| {
            handle_connection(stream, msg_vec_clone);
        });
    }
    println!("Shutting down.");
}
