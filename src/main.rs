use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
mod http_response;
mod http_request;
use http_response::HTTPResponse;
use http_request::HTTPRequest;
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
    let mut request = HTTPRequest::new();
    

    let mut buf_reader = BufReader::new(&mut stream);
    request.set_from_buffer(&mut buf_reader);
    
    let mut response: Option<HTTPResponse> = None;

    if request.endpoint().starts_with("/"){
        if request.endpoint() == "/" {
            response = Some(HTTPResponse::new("HTTP/1.1 200 OK".to_string()));
        }else if request.endpoint().starts_with("/echo/"){
            response = Some(HTTPResponse::new("HTTP/1.1 200 OK".to_string()));
            let url: Vec<&str> = request.endpoint().split("/echo/").collect();
            if let Some(resp) = response.as_mut() {
                resp.set_body_as_plain_text(url[1].to_string());
            }
        }else if request.endpoint() == "/user-agent"{
            response = Some(HTTPResponse::new("HTTP/1.1 200 OK".to_string()));
            if let Some(resp) = response.as_mut() {
                let headers = request.headers();

                resp.set_body_as_plain_text(headers["User-Agent"].clone());
            }
        }else{
            response  = Some(HTTPResponse::new("HTTP/1.1 404 Not Found".to_string()));
        }

    }else{
        response  = Some(HTTPResponse::new("HTTP/1.1 404 Not Found".to_string()));
    };

    
    stream.write_all(response.unwrap().get_formatted_response().as_bytes()).unwrap();
}