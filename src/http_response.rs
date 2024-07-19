#[derive(Clone)]
pub struct HTTPResponse {
    status_line: String,
    headers: Option<HTTPHeaders>,
    body: Option<HTTPBody>, 
}

impl Default for HTTPResponse {
    fn default() -> Self {
        Self {
            status_line: String::new(),
            headers: None,
            body: None,
        }
    }
}

impl HTTPResponse {
    pub fn new(status_line: String) -> Self {
        Self { status_line, ..Default::default()}
    }

    pub fn set_body_as_plain_text(&mut self, content_type: String, body: String) {
        let headers =  HTTPHeaders{
            content_length: Some(body.as_bytes().len().to_string()),
            content_type: Some(content_type),
        };
        let body = HTTPBody {
            body: body,
        };

        self.headers = Some(headers);
        self.body = Some(body);
    }

    pub fn get_formatted_response(&self) -> String {
        let mut result: String= format!("{0}\r\n", self.status_line);

        if let Some(headers) = &self.headers{
            result.push_str(&format!("{0}", headers.get_formatted() ))
        };
        result.push_str("\r\n");

        if let Some(body) = &self.body{
            result.push_str(&format!("{0}", body.get_formatted()))
        };

        return result;
    }


}

#[derive(Clone)]
struct HTTPHeaders {
    content_type: Option<String>,
    content_length: Option<String>,
}

impl Default for HTTPHeaders {
    fn default() -> Self {
        Self {
            content_type: None,
            content_length: None,
        }
    }
}

impl HTTPHeaders {
    fn new() -> Self {
        Self {..Default::default()}
    }

    pub fn set_content_type(mut self, content_type: String){
        self.content_type = Some(content_type);
    }

    pub fn set_content_length(mut self, content_length: String){
        self.content_length = Some(content_length);
    }


    pub fn get_formatted(&self) -> String {
        //self.content_length.is_some()
        let mut result: String= String::from("");

        if let Some(content_type) = &self.content_type{
            result.push_str(&format!("Content-Type: {content_type}\r\n"))
        };
        if let Some(content_length) = &self.content_length{
            result.push_str(&format!("Content-Length: {content_length}\r\n"))
        };
        return result;

    }


}

#[derive(Clone)]
struct HTTPBody {
    body: String,
}

impl HTTPBody {
    fn new(body: String) -> Self {
        Self {body}
    }
    fn get_formatted(&self) -> String {
        return format!("{0}", self.body)
    }

}