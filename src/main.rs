use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::process;
use std::env;

use rand::Rng;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn get_files(path: &str) -> Result<Vec<PathBuf>, std::io::Error> {

    let entries = fs::read_dir(path)?;

    let mut files = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            files.push(path);
        }
    }

    Ok(files)

}

fn didnt_feel_like_it(mut stream: TcpStream) {

    let mut buff = [0; 1024];

    let _ = stream.read(&mut buff);

    let _ = String::from_utf8_lossy(&buff);

    let mut res = String::from("HTTP/1.1 404 Not Found");
    res.push_str("Server: LazyHTTPServer\n");
    res.push_str("Content-Type: text/html;\n\n");   

    res.push_str("<h1> Error 404: Didn't feel like it </h1> <h3> I didn't feel like sending anything. Sue me </h3>");

    //println!("Sending:\n\n{}", res);

    let _ = stream.write(res.as_bytes());
}

fn handle_client(mut stream: TcpStream) {

    let mut buff = [0; 1024];

    let _ = stream.read(&mut buff);

    let _ = String::from_utf8_lossy(&buff);

    let mut res = String::from("HTTP/1.1 200 OK\n");
    res.push_str("Server: LazyHTTPServer\n");
    res.push_str("Content-Type: text/html;\n\n");

    let files = get_files("./");

    match files {
        Ok(files) => {
            let mut rng = thread_rng();
            let html_files: Vec<_> = files.into_iter().filter(|f| f.file_name().and_then(|name| name.to_str()).map(|name| name.ends_with(".html")).unwrap()).collect();
            let random_file = html_files.choose(&mut rng).unwrap();

            let contents = match fs::read_to_string(random_file) {
                Ok(contents) => contents,
                Err(_) => "I didn't feel like reading your file".to_string()
            };

            res.push_str(&contents);
            
            //println!("Sending:\n\n{}", res);

            let _ = stream.write(res.as_bytes());

        },
        Err(_) => eprintln!("Failed to get files")
    }

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
                let mut rng = thread_rng();
                let chance_of_sending = rng.gen_range(1..=100);
                if chance_of_sending <= 33 {
                    thread::spawn(|| handle_client(stream));
                } else {
                    thread::spawn(|| didnt_feel_like_it(stream));
                }
            },
            Err(e) => eprintln!("{}", e)
        }
    }
}
