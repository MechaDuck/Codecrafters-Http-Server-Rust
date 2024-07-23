use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    fs::File,
    io::{self, Read},
    path::Path,
};
use flate2::write::GzEncoder;
use flate2::Compression;

use clap::Parser;
use base64;

mod http_response;
mod http_request;
use http_response::HTTPResponse;
use http_request::HTTPRequest;
use nom::AsChar;

// Define an enum for compression algorithms
enum CompressionAlgorithm {
    Gzip,
    Invalid_Encoding,
}

// Define command-line arguments structure using `clap`
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to file storage directory
    #[arg(short, long)]
    directory: Option<String>,
}

fn main() {
    // Parse command-line arguments
    let args = Args::parse();
    let directory = args.directory.clone();

    if let Some(ref dir) = directory {
        println!("Value for directory: {}", dir);
    }

    println!("Logs from your program will appear here!");

    // Bind the TCP listener to address 127.0.0.1:4221
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    // Accept incoming TCP connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let directory = directory.clone();
                // Spawn a new thread to handle each connection
                thread::spawn(move || {
                    println!("accepted new connection");
                    handle_connection(stream, directory);
                });
            }
            Err(e) => println!("error: {}", e),
        }
    }
}

// Handle incoming HTTP connection
fn handle_connection(mut stream: TcpStream, file_directory: Option<String>) {
    let mut request = HTTPRequest::new();
    let mut buf_reader = BufReader::new(&mut stream);
    // Parse HTTP request from the buffer
    request.set_from_buffer(&mut buf_reader);

    // Match request method and generate appropriate response
    let response = match request.method() {
        "GET" => handle_get_request(request, file_directory),
        "POST" => handle_post_request(request, file_directory),
        _ => HTTPResponse::new("HTTP/1.1 404 Not Found".to_string()),
    };

    // Write response back to the client
    stream.write_all(&response.get_formatted_response()).unwrap();
}

// Handle POST requests
fn handle_post_request(request: HTTPRequest, file_directory: Option<String>) -> HTTPResponse {
    let response = match request.endpoint() {
        endpoint if endpoint.starts_with("/files/") => handle_file_post(endpoint, &file_directory, request.body()),
        _ => HTTPResponse::new("HTTP/1.1 404 Not Found".to_string()),
    };
    response
}

// Handle GET requests
fn handle_get_request(request: HTTPRequest, file_directory: Option<String>) -> HTTPResponse {
    let response = match request.endpoint() {
        "/" => handle_root(),
        endpoint if endpoint.starts_with("/echo/") => handle_echo(request),
        "/user-agent" => handle_user_agent(&request),
        endpoint if endpoint.starts_with("/files/") => handle_file_request(endpoint, &file_directory),
        _ => HTTPResponse::new("HTTP/1.1 404 Not Found".to_string()),
    };
    response
}

// Handle root ("/") request
fn handle_root() -> HTTPResponse {
    HTTPResponse::new("HTTP/1.1 200 OK".to_string())
}

// Handle "/echo/" requests
fn handle_echo(request: HTTPRequest) -> HTTPResponse {
    let response_text = request.endpoint().trim_start_matches("/echo/");
    let mut response = HTTPResponse::new("HTTP/1.1 200 OK".to_string());



    // Set Content-Encoding header if requested
    if request.headers().contains_key("Accept-Encoding") {
        // Split the string by commas and trim whitespace
        let encodings_vec: Vec<&str> = request.headers()["Accept-Encoding"]
        .split(',')
        .map(|s| s.trim())
        .collect();
        if encodings_vec.contains(&"gzip") {
            response.get_headers().set_content_encoding("gzip".to_string());
            match gzip_data(response_text){
                Ok(compressed_data) => {
                    response.set_body_as_plain_text("text/plain".to_string(), compressed_data);
                    return response;

                }
                Err(e) => {
                    eprintln!("Failed to compress data: {}", e);
                }

            }
        }
    }
    response.set_body_as_plain_text("text/plain".to_string(), response_text.as_bytes().to_vec());
    response
}

// Handle "/user-agent" requests
fn handle_user_agent(request: &HTTPRequest) -> HTTPResponse {
    let mut response = HTTPResponse::new("HTTP/1.1 200 OK".to_string());
    if let Some(user_agent) = request.headers().get("User-Agent") {
        response.set_body_as_plain_text("text/plain".to_string(), user_agent.clone().as_bytes().to_vec());
    }
    response
}

// Handle file requests
fn handle_file_request(endpoint: &str, file_directory: &Option<String>) -> HTTPResponse {
    if let Some(ref dir) = *file_directory {
        let filename: &str = endpoint.trim_start_matches("/files/");
        match read_file_as_string(dir, filename) {
            Ok(content) => {
                let mut response = HTTPResponse::new("HTTP/1.1 200 OK".to_string());
                response.set_body_as_plain_text("application/octet-stream".to_string(), content.as_bytes().to_vec());
                response
            }
            Err(_) => HTTPResponse::new("HTTP/1.1 404 Not Found".to_string()),
        }
    } else {
        HTTPResponse::new("HTTP/1.1 404 Not Found".to_string())
    }
}

// Read file contents into a string
fn read_file_as_string(dir: &str, filename: &str) -> Result<String, io::Error> {
    let path = Path::new(dir).join(filename);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Handle file POST requests
fn handle_file_post(endpoint: &str, file_directory: &Option<String>, content: &str) -> HTTPResponse {
    if let Some(ref dir) = *file_directory {
        let filename: &str = endpoint.trim_start_matches("/files/");
        match write_string_to_file(dir, filename, content) {
            Ok(_) => HTTPResponse::new("HTTP/1.1 201 Created".to_string()),
            Err(_) => HTTPResponse::new("HTTP/1.1 404 Not Found".to_string()),
        }
    } else {
        HTTPResponse::new("HTTP/1.1 404 Not Found".to_string())
    }
}

// Write string content to a file
fn write_string_to_file(dir: &str, filename: &str, content: &str) -> Result<(), io::Error> {
    let path = Path::new(dir).join(filename);
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn gzip_data(data: &str) -> io::Result<Vec<u8>>{
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data.as_bytes())?;
    let compressed_data = encoder.finish()?;
    Ok(compressed_data)
}