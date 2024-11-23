mod utils;

use runserver::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};

const RESPONSE_200_LINE: &str = "HTTP/1.1 200 OK";
const RESPONSE_404_LINE: &str = "HTTP/1.1 404 NOT FOUND";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (_method, path, _version) = utils::parse_request_line(&request_line);

    let (status_line, filename) = match path {
        "/" => (RESPONSE_200_LINE, "hello.html"),
        "/sleep" => {
            thread::sleep(std::time::Duration::from_secs(5));
            (RESPONSE_200_LINE, "hello.html")
        }
        _ => (RESPONSE_404_LINE, "404.html"),
    };

    let contents = fs::read_to_string(format!("pages/{filename}")).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
