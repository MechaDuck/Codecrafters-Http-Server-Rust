use std::{
    io::{prelude::*, BufReader, BufRead,Cursor},
    net::{TcpListener, TcpStream},
};
use std::collections::HashMap;

pub struct HTTPRequest{
    method: String,
    endpoint: String,
    protocol: String,
    headers: HashMap<String, String>,

}


impl HTTPRequest{

    pub fn new() -> Self {
        HTTPRequest {
            method: String::new(),
            endpoint: String::new(),
            protocol: String::new(),
            headers: HashMap::new(),
        }
    }

    pub fn set_from_buffer(&mut self, buffer: &mut dyn BufRead){
        let mut lines = buffer.lines();

        if let Some(Ok(request_line)) = lines.next() {
            self.set_from_request_line(request_line);
        }

        for line in lines {
            match line {
                Ok(header_line) => {
                    if header_line.trim().is_empty() {
                        break;
                    }
                    let header_elements: Vec<&str> = header_line.split(": ").collect();
                    self.headers.insert(header_elements[0].to_string(), header_elements[1].to_string());
                }
                Err(e) => {
                    eprintln!("Error reading line: {}", e);
                }
            }
        }

    }

    fn set_from_request_line(&mut self, line: String){
        let request_line: Vec<&str> = line.split(' ').collect();
        self.method = request_line[0].to_string();
        self.endpoint = request_line[1].to_string();
        self.protocol = request_line[2].to_string();
    }
    // Getters for testing
    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn protocol(&self) -> &str {
        &self.protocol
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_set_from_buffer() {
        let mut request = HTTPRequest::new();
        let input = "GET /index HTTP/1.1\r\nHost: example.com\r\nUser-Agent: curl/7.68.0\r\n\r\n";
        let mut buffer: Cursor<&str> = Cursor::new(input);


        request.set_from_buffer(&mut buffer);

        assert_eq!(request.method(), "GET");
        assert_eq!(request.endpoint(), "/index");
        assert_eq!(request.protocol(), "HTTP/1.1");

        let headers = request.headers();
        assert_eq!(headers.len(), 2);
        assert_eq!(headers["Host"], "example.com");
        assert_eq!(headers["User-Agent"], "curl/7.68.0");
    }

    #[test]
    fn test_set_from_request_line() {
        let mut request = HTTPRequest::new();
        let request_line = "POST /submit HTTP/1.0";
        let mut buffer: Cursor<&str> = Cursor::new(request_line);

        request.set_from_buffer(&mut buffer);

        assert_eq!(request.method(), "POST");
        assert_eq!(request.endpoint(), "/submit");
        assert_eq!(request.protocol(), "HTTP/1.0");
    }
}