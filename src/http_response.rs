#[derive(Clone)]
pub struct HTTPResponse {
    status_line: String,        // Status line of the HTTP response (e.g., "HTTP/1.1 200 OK")
    headers: HTTPHeaders,       // HTTP headers of the response
    body: Option<HTTPBody>,     // Optional body of the response
}

impl Default for HTTPResponse {
    fn default() -> Self {
        Self {
            status_line: String::new(),
            headers: HTTPHeaders::new(),
            body: None,
        }
    }
}

impl HTTPResponse {
    // Create a new HTTPResponse with a specified status line
    pub fn new(status_line: String) -> Self {
        Self { status_line, ..Default::default() }
    }

    // Set the body of the response as plain text and update headers accordingly
    pub fn set_body_as_plain_text(&mut self, content_type: String, body: String) {
        self.headers.set_content_length(body.as_bytes().len().to_string());
        self.headers.set_content_type(content_type.clone());

        let body = HTTPBody {
            body: body,
        };

        self.body = Some(body);
    }

    // Format the entire HTTP response as a string
    pub fn get_formatted_response(&self) -> String {
        let mut result: String = format!("{0}\r\n", self.status_line);

        result.push_str(&format!("{0}", self.headers.get_formatted()));
        
        result.push_str("\r\n");

        if let Some(body) = &self.body {
            result.push_str(&format!("{0}", body.get_formatted()));
        }

        result
    }

    // Return a mutable reference to the HTTPHeaders
    pub fn get_headers(&mut self) -> &mut HTTPHeaders {
        &mut self.headers
    }
}

#[derive(Clone)]
pub struct HTTPHeaders {
    content_type: Option<String>,       // Content-Type header
    content_length: Option<String>,     // Content-Length header
    content_encoding: Option<String>,   // Content-Encoding header
}

impl Default for HTTPHeaders {
    fn default() -> Self {
        Self {
            content_type: None,
            content_length: None,
            content_encoding: None,
        }
    }
}

impl HTTPHeaders {
    // Create a new HTTPHeaders instance with default values
    fn new() -> Self {
        Self { ..Default::default() }
    }

    // Set the Content-Type header
    pub fn set_content_type(&mut self, content_type: String) {
        self.content_type = Some(content_type);
    }

    // Set the Content-Length header
    pub fn set_content_length(&mut self, content_length: String) {
        self.content_length = Some(content_length);
    }

    // Set the Content-Encoding header
    pub fn set_content_encoding(&mut self, content_encoding: String) {
        self.content_encoding = Some(content_encoding);
    }

    // Format the headers as a string
    pub fn get_formatted(&self) -> String {
        let mut result: String = String::new();

        if let Some(content_type) = &self.content_type {
            result.push_str(&format!("Content-Type: {content_type}\r\n"));
        }
        if let Some(content_length) = &self.content_length {
            result.push_str(&format!("Content-Length: {content_length}\r\n"));
        }
        if let Some(content_encoding) = &self.content_encoding {
            result.push_str(&format!("Content-Encoding: {content_encoding}\r\n"));
        }
        result
    }
}

#[derive(Clone)]
struct HTTPBody {
    body: String,   // The body of the HTTP response
}

impl HTTPBody {
    // Create a new HTTPBody with the provided body content
    fn new(body: String) -> Self {
        Self { body }
    }

    // Format the body content as a string
    fn get_formatted(&self) -> String {
        format!("{}", self.body)
    }
}
