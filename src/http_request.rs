use std::{
    io::{self, prelude::*, BufReader, BufRead},
    net::{TcpListener, TcpStream},
};
use std::collections::HashMap;

/// Represents an HTTP request.
pub struct HTTPRequest {
    method: String,        // HTTP method (e.g., "GET", "POST")
    endpoint: String,      // Request endpoint (e.g., "/index")
    protocol: String,      // HTTP protocol version (e.g., "HTTP/1.1")
    headers: HashMap<String, String>,  // HTTP headers
    body: String,          // Body of the HTTP request
}

impl HTTPRequest {
    /// Creates a new, empty HTTPRequest.
    pub fn new() -> Self {
        HTTPRequest {
            method: String::new(),
            endpoint: String::new(),
            protocol: String::new(),
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    /// Parses the HTTP request from the provided buffer.
    ///
    /// The buffer should contain the request line, headers, and optionally the body.
    pub fn set_from_buffer(&mut self, buffer: &mut dyn BufRead) -> io::Result<()> {
        // Read and parse the request line (e.g., "GET /index HTTP/1.1")
        let mut request_line = String::new();
        buffer.read_line(&mut request_line)?;
        self.set_from_request_line(request_line.trim().to_string());

        // Read and parse headers
        let mut header_line = String::new();
        while buffer.read_line(&mut header_line)? > 0 {
            let trimmed_line = header_line.trim();
            if trimmed_line.is_empty() {
                break; // End of headers
            }
            let mut header_parts = trimmed_line.splitn(2, ": ");
            if let (Some(key), Some(value)) = (header_parts.next(), header_parts.next()) {
                self.headers.insert(key.to_string(), value.to_string());
            }
            header_line.clear(); // Clear the buffer for the next line
        }

        // Read the body if Content-Length is specified
        if let Some(content_length) = self.headers.get("Content-Length") {
            let content_length: usize = content_length.parse().unwrap_or(0);
            let mut body_buffer = vec![0; content_length];
            buffer.read_exact(&mut body_buffer).unwrap();
            self.body = String::from_utf8(body_buffer)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        }

        Ok(())
    }

    /// Parses and sets the request line (method, endpoint, protocol) from the given line.
    fn set_from_request_line(&mut self, line: String) {
        let request_line: Vec<&str> = line.split(' ').collect();
        self.method = request_line[0].to_string();
        self.endpoint = request_line[1].to_string();
        self.protocol = request_line[2].to_string();
    }

    // Getters for various parts of the request

    /// Returns the HTTP method (e.g., "GET", "POST").
    pub fn method(&self) -> &str {
        &self.method
    }

    /// Returns the request endpoint (e.g., "/index").
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Returns the HTTP protocol version (e.g., "HTTP/1.1").
    pub fn protocol(&self) -> &str {
        &self.protocol
    }

    /// Returns a reference to the headers of the request.
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Returns the body of the request.
    pub fn body(&self) -> &str {
        &self.body
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

        // Parse the request from the buffer
        request.set_from_buffer(&mut buffer).unwrap();

        // Assert that the method, endpoint, and protocol are parsed correctly
        assert_eq!(request.method(), "GET");
        assert_eq!(request.endpoint(), "/index");
        assert_eq!(request.protocol(), "HTTP/1.1");

        // Assert that the headers are parsed correctly
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

        // Parse the request line from the buffer
        request.set_from_buffer(&mut buffer).unwrap();

        // Assert that the method, endpoint, and protocol are parsed correctly
        assert_eq!(request.method(), "POST");
        assert_eq!(request.endpoint(), "/submit");
        assert_eq!(request.protocol(), "HTTP/1.0");
    }
}
