use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) -> () {
    //NOTE: we create a new BufReader instance that wraps a mutable reference to the stream.
    //BufReader adds buffering by managing calls to the std::io::Read trait methods for us.
    let buf_reader = BufReader::new(&mut stream);

    //NOTE: BufReader implements the std::io::BufRead trait, which provides the lines method.
    //The lines method returns an iterator of Result<String, std::io::Error> by splitting the
    //stream of data whenever it sees a newline byte. To get each String, we map and unwrap each Result
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    // println!("Request: {:#?}", http_request);
    // let response = "HTTP/1.1 200 OK\r\n\r\n";

    // let status_line = "HTTP/1.1 200 OK";
    // let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    //NOTE: The write_all method on stream takes a &[u8] and sends those bytes directly down the connection.
    //Because the write_all operation could fail, we use unwrap on any error result as before.
    //Again, in a real application you would add error handling here.
    stream.write_all(response.as_bytes()).unwrap();
}
