use muths::ThreadPool;
use std::{
    error::Error,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("localhost:8787").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            pool.execute(|| {
                let addr = stream.peer_addr().unwrap();
                if let Err(err) = handle_connection(stream) {
                    println!("Got error {:#?} serving {:#?}", err, addr);
                }
            });
        } else {
            println!("Exiting");
            return;
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = match buf_reader.lines().next() {
        Some(rl) => rl?,
        None => return Ok(()), // ignore empty requests
    };

    let (status_line, contents) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200", include_str!("../hello.html")),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404", "Not Found"),
    };

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes())?;

    println!("Req: {:#?} -> {:#?}", request_line, status_line);
    Ok(())
}
