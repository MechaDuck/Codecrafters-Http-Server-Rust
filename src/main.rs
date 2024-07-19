use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};
use std::io::{self, Read};
use std::path::Path;
use std::fs::File;

mod http_response;
mod http_request;
use http_response::HTTPResponse;
use http_request::HTTPRequest;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    /// path to file storage directory
    #[arg(short, long)]
    directory: Option<String>,
}

fn main() {
    // Reading command flags
    let args = Args::parse();

    // check for directory flag
    let directory = args.directory.clone();
    if let Some(ref dir) = directory {
        println!("Value for directory: {}", dir);
    }

    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        let directory = directory.clone();
        match stream {
            Ok(mut _stream) => {
                thread::spawn(move || {
                    println!("accepted new connection");
                    handle_connection(_stream, directory);
                });

            }
            Err(e) => {
                println!("error: {}", e);
            }

        }
    }
}

fn handle_connection(mut stream: TcpStream, file_directory: Option<String>) {
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
                resp.set_body_as_plain_text("text/plain".to_string(), url[1].to_string());
            }
        }else if request.endpoint() == "/user-agent"{
            response = Some(HTTPResponse::new("HTTP/1.1 200 OK".to_string()));
            if let Some(resp) = response.as_mut() {
                let headers = request.headers();

                resp.set_body_as_plain_text("text/plain".to_string(), headers["User-Agent"].clone());
            }
        }else if request.endpoint().starts_with("/files/"){
            if file_directory.is_some(){
                let filename: Vec<&str> = request.endpoint().split("/files/").collect();
                let content = read_file_as_string(file_directory.unwrap(), filename[1].to_string());
                if content.is_ok(){
                    response = Some(HTTPResponse::new("HTTP/1.1 200 OK".to_string()));
                    if let Some(resp) = response.as_mut() {
                        resp.set_body_as_plain_text("application/octet-stream".to_string(), content.unwrap());
                    }
                }else {
                    response  = Some(HTTPResponse::new("HTTP/1.1 404 Not Found".to_string()));

                }
            }else{
                response  = Some(HTTPResponse::new("HTTP/1.1 404 Not Found".to_string()));
            }
            
        }else{
            response  = Some(HTTPResponse::new("HTTP/1.1 404 Not Found".to_string()));
        }

    }else{
        response  = Some(HTTPResponse::new("HTTP/1.1 404 Not Found".to_string()));
    };

    
    stream.write_all(response.unwrap().get_formatted_response().as_bytes()).unwrap();
}


fn read_file_as_string(dir: String, filename: String) -> Result<String, io::Error>{
    let path_string = format!("{dir}{filename}");
    let path = Path::new(path_string.as_str() );

    let mut file = File::options()
        .read(true)
        .write(true)
        .open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    Ok(contents)
}