use std::collections::HashMap;

#[derive(Debug)]
pub struct Response {
    status: usize,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Response {
    pub fn new() -> Self {
        Self {
            status: 200,
            headers: HashMap::<String, String>::new(),
            body: Vec::<u8>::new(),
        }
    }

    pub fn set_header(&mut self, k: &str, v: &str) {
        self.headers.insert(k.to_string(), v.to_string());
    }

    pub fn set_body(&mut self, body: &str) {
        self.body = body.as_bytes().to_vec();
    }

    pub fn set_status(&mut self, status: usize) {
        self.status = status;
    }

    pub fn build_response(&self) -> Vec<u8> {
        let protocol = "HTTP/1.1";
        let status = self.status;
        let headers = self.headers.iter().fold(String::new(), |acc, (k, v)| {
            acc + &format!("{}: {}\r\n", k, v)
        });
        let body = String::from_utf8_lossy(&self.body[..]);
        let formatted = format!("{} {}\r\n{}\r\n\r\n{}\r\n", protocol, status, headers, body);
        formatted.as_bytes().to_vec()
    }
}
