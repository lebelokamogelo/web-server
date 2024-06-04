// TODO: Building a single threaded web server
//
// Listen to incoming request and
// return a corresponding response
// and html content
//
use std::{
    fs,
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

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // convert the stream data into buffer
    // to make it easy to work with
    let buffer = BufReader::new(&stream);

    // iter over the buffer then get the first line
    // more likely the header
    // e.g Request: "GET / HTTP/1.1"
    let request_line = buffer.lines().next().unwrap().unwrap();

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
    // let status = "HTTP/1.1 200 OK";

    // return the reponse to the client
    // convert it to bytes, tcp transport is a byte stream protocol
    // stream.write_all(response.as_bytes()).unwrap();

    // TODO: Returning Real html
    // use the file system to get the content of the file
    //
    //
    // TODO: Validating the request and selectively responding
    //
    //
    /* ### OLD CODE
    if http_request.starts_with("GET / HTTP/1.1") {
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        # format the response
        let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");

        # render the content to the browser on each request
        stream.write(response.as_bytes()).unwrap();
    } else {
        # return the 404.html Not Found
        let contents = fs::read_to_string("404.html").unwrap();

        let length = contents.len();

        let status = "HTTP/1.1 404 NOT FOUND";
        let reponse = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write(reponse.as_bytes()).unwrap();
    }
    */

    // Touch of refactoring
    // remove the code repetition
    // clean up
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
