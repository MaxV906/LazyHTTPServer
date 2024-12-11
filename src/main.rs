use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::process;
use std::env;

fn handle_client(mut stream: TcpStream) {

    let mut buff = [0; 1024];

    stream.read(&mut buff).expect("Failed to read data from client");

    let req = String::from_utf8_lossy(&buff);

    let mut res = String::from("HTTP/1.1 200 OK\n");

    res.push_str("Server: LazyHTTPServer\n");
    res.push_str("Content-Type: text/html;\n\n");

    res.push_str("Hello");

    let _ = stream.write(res.as_bytes());

}

fn main() {

    let args: Vec<String> = env::args().collect();
    let ip_addr = &args[1]; 
    let port = &args[2]; 

    if args.len() < 3 {
        eprintln!("Usage: {} [ip address] [port]", args[0]);
        process::exit(1);
    }

    let http_server = TcpListener::bind(format!("{}:{}", *ip_addr, *port))
        .expect(&format!("Failed to bind to port {}", *port));

    println!("Listening for connections on: {}:{}", *ip_addr, *port);

    for stream in http_server.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            },
            Err(e) => eprintln!("{}", e)
        }
    }
}
