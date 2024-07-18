use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
mod http_response;
use http_response::HTTPResponse;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                handle_connection(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }

        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let endpoint_unformatted: Vec<&str> = request_line.split(' ').collect();
    let endpoint = endpoint_unformatted[1];
    let mut response: Option<HTTPResponse> = None;

    if endpoint.starts_with("/"){
        if endpoint == "/" {
            response = Some(HTTPResponse::new("HTTP/1.1 200 OK".to_string()));
        }else if endpoint.starts_with("/echo/"){
            response = Some(HTTPResponse::new("HTTP/1.1 200 OK".to_string()));
            let url: Vec<&str> = endpoint.split("/echo/").collect();
            if let Some(resp) = response.as_mut() {
                resp.set_body_as_plain_text(url[1].to_string());
            }
        }else{
            response  = Some(HTTPResponse::new("HTTP/1.1 404 Not Found".to_string()));
        }

    }else{
        response  = Some(HTTPResponse::new("HTTP/1.1 404 Not Found".to_string()));
    };

    
    stream.write_all(response.unwrap().get_formatted_response().as_bytes()).unwrap();
}