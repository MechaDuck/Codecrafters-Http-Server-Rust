use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    fs::File,
    io::{self, Read},
    path::Path,
};
use clap::Parser;

mod http_response;
mod http_request;
use http_response::HTTPResponse;
use http_request::HTTPRequest;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    /// path to file storage directory
    #[arg(short, long)]
    directory: Option<String>,
}

fn main() {
    let args = Args::parse();
    let directory = args.directory.clone();

    if let Some(ref dir) = directory {
        println!("Value for directory: {}", dir);
    }

    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let directory = directory.clone();
                thread::spawn(move || {
                    println!("accepted new connection");
                    handle_connection(stream, directory);
                });
            }
            Err(e) => println!("error: {}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream, file_directory: Option<String>) {
    let mut request = HTTPRequest::new();
    let mut buf_reader = BufReader::new(&mut stream);
    request.set_from_buffer(&mut buf_reader);

    let response = match request.method() {
        "GET" => handle_get_request(request,file_directory),
        "POST" => handle_post_request(request,file_directory),
        _ => HTTPResponse::new("HTTP/1.1 404 Not Found".to_string()),
    };

    stream.write_all(response.get_formatted_response().as_bytes()).unwrap();
}
fn handle_post_request(request: HTTPRequest,file_directory: Option<String>) -> HTTPResponse {
    let response = match request.endpoint() {
        endpoint if endpoint.starts_with("/files/") => handle_file_post(endpoint, &file_directory, request.body()),
        _=> HTTPResponse::new("HTTP/1.1 404 Not Found".to_string()),
    };
    response
}
fn handle_get_request(request: HTTPRequest,file_directory: Option<String>) -> HTTPResponse {
    let response = match request.endpoint() {
        "/" => handle_root(),
        endpoint if endpoint.starts_with("/echo/") => handle_echo(endpoint),
        "/user-agent" => handle_user_agent(&request),
        endpoint if endpoint.starts_with("/files/") => handle_file_request(endpoint, &file_directory),
        _ => HTTPResponse::new("HTTP/1.1 404 Not Found".to_string()),
    };
    response
}

fn handle_root() -> HTTPResponse {
    HTTPResponse::new("HTTP/1.1 200 OK".to_string())
}

fn handle_echo(endpoint: &str) -> HTTPResponse {
    let response_text = endpoint.trim_start_matches("/echo/");
    let mut response = HTTPResponse::new("HTTP/1.1 200 OK".to_string());
    response.set_body_as_plain_text("text/plain".to_string(), response_text.to_string());
    response
}

fn handle_user_agent(request: &HTTPRequest) -> HTTPResponse {
    let mut response = HTTPResponse::new("HTTP/1.1 200 OK".to_string());
    if let Some(user_agent) = request.headers().get("User-Agent") {
        response.set_body_as_plain_text("text/plain".to_string(), user_agent.clone());
    }
    response
}

fn handle_file_request(endpoint: &str, file_directory: &Option<String>) -> HTTPResponse {
    if let Some(ref dir) = *file_directory {
        let filename: &str = endpoint.trim_start_matches("/files/");
        match read_file_as_string(dir, filename) {
            Ok(content) => {
                let mut response = HTTPResponse::new("HTTP/1.1 200 OK".to_string());
                response.set_body_as_plain_text("application/octet-stream".to_string(), content);
                response
            }
            Err(_) => HTTPResponse::new("HTTP/1.1 404 Not Found".to_string()),
        }
    } else {
        HTTPResponse::new("HTTP/1.1 404 Not Found".to_string())
    }
}

fn read_file_as_string(dir: &str, filename: &str) -> Result<String, io::Error> {
    let path = Path::new(dir).join(filename);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn handle_file_post(endpoint: &str, file_directory: &Option<String>, content: &str) -> HTTPResponse {
    if let Some(ref dir) = *file_directory {
        let filename: &str = endpoint.trim_start_matches("/files/");
        match write_string_to_file(dir, filename, content) {
            Ok(content) => {
                let mut response = HTTPResponse::new("HTTP/1.1 201 Created".to_string());
                response
            }
            Err(_) => HTTPResponse::new("HTTP/1.1 404 Not Found".to_string()),
        }
    } else {
        HTTPResponse::new("HTTP/1.1 404 Not Found".to_string())
    }
}

fn write_string_to_file(dir: &str, filename: &str, content: &str) -> Result<(), io::Error> {
    let path = Path::new(dir).join(filename);
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
