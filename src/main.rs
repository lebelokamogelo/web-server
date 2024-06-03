// TODO: Building a single threaded web server
//
// Listen to incoming request and
// return a corresponding response
// and html content
//
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // create the TcpListener
    // bind it to port 7878

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // console a connection success message
    println!("Connecitons established!");

    // Listening to the incoming request
    for stream in listener.incoming() {
        // stream return a results
        let stream = stream.unwrap();

        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    // convert the stream data into buffer
    // to make it easy to work with
    let buffer = BufReader::new(&stream);

    // iter over the buffer then get the first line
    // more likely the header
    // e.g Request: "GET /hhh HTTP/1.1"
    let http_request = buffer.lines().next().unwrap().unwrap();

    // A Closer Look at an HTTP Request
    //HTTP is a text-based protocol, and a request takes this format:

    //Method Request-URI HTTP-Version CRLF(\r\n)
    //headers CRLF(\r\m)
    //message-body
    //

    // TODO: Writing a response
    // format
    // HTTP-Version Status-Code Reason-Phrase CRLF
    //headers CRLF
    //message-body
    //
    //e.g simple response HTTP/1.1 200 OK\r\n\r\n
    //
    // writing a simple http response with no body
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    // return thr reponse to the client
    // convert it to bytes since the tcp transport is a byte stream protocol
    stream.write_all(response.as_bytes()).unwrap();
}
