use std::{collections::HashMap, fmt::Display, str::Lines};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Options,
    Error,
}

impl HttpMethod {
    pub fn from(method: &str) -> Self {
        match method.to_lowercase().as_str() {
            "get" => Self::Get,
            "post" => Self::Post,
            "put" => Self::Put,
            "patch" => Self::Patch,
            "delete" => Self::Delete,
            "options" => Self::Options,
            _ => Self::Error,
        }
    }
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Get => f.write_str("Get"),
            Self::Post => f.write_str("Post"),
            Self::Put => f.write_str("Put"),
            Self::Patch => f.write_str("Patch"),
            Self::Delete => f.write_str("Delete"),
            Self::Options => f.write_str("Options"),
            Self::Error => f.write_str("_"),
        }
    }
}

struct RequestData {
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub body: Vec<u8>,
    pub method: HttpMethod,
    pub route: String,
}

impl RequestData {
    pub fn new(
        headers: HashMap<String, String>,
        query_params: HashMap<String, String>,
        body: Vec<u8>,
        method: HttpMethod,
        route: String,
    ) -> Self {
        Self {
            headers,
            query_params,
            body,
            method,
            route,
        }
    }
}

pub struct Request {
    method: HttpMethod,
    route: String,
    headers: HashMap<String, String>,
    url_params: HashMap<String, String>,
    query_params: HashMap<String, String>,
    body: Vec<u8>,
}

impl Request {
    pub fn new(buffer: [u8; 1024]) -> Self {
        let data = Self::parse_request(buffer);
        Self {
            headers: data.headers,
            body: data.body,
            method: data.method,
            route: data.route,
            query_params: data.query_params,
            url_params: HashMap::<String, String>::new(),
        }
    }

    pub fn get_header(&self, k: &str) -> Option<&String> {
        self.headers.get(k)
    }

    pub fn body(&self) -> &[u8] {
        &self.body
    }

    pub fn route(&self) -> &String {
        &self.route
    }

    pub fn method(&self) -> &HttpMethod {
        &self.method
    }

    pub fn set_url_params(&mut self, params: Option<HashMap<String, String>>) {
        if let Some(params) = params {
            self.url_params = params;
        }
    }

    pub fn get_url_param(&self, param: &str) -> Option<&String> {
        self.url_params.get(param)
    }

    pub fn set_query_params(&mut self, params: Option<HashMap<String, String>>) {
        if let Some(params) = params {
            self.query_params = params;
        }
    }

    pub fn get_query_params(&self, param: &str) -> Option<&String> {
        self.query_params.get(param)
    }

    fn parse_request(buffer: [u8; 1024]) -> RequestData {
        let string = String::from_utf8_lossy(&buffer[..]);

        let mut lines = string.lines();
        let status_line = if let Some(line) = lines.next() {
            line
        } else {
            ""
        };

        let headers = Self::parse_headers(&mut lines);
        let (method, route) = Self::parse_status_line(status_line);
        let query_params = Self::parse_query_params(&route);
        let body = lines.collect::<Vec<&str>>().join("\n").as_bytes().to_vec();
        RequestData::new(headers, query_params, body, method, route)
    }

    fn parse_status_line(line: &str) -> (HttpMethod, String) {
        let split: Vec<&str> = line.split_whitespace().collect();
        if let (Some(method), Some(route)) = (split.get(0), split.get(1)) {
            (HttpMethod::from(method), route.to_string())
        } else {
            (HttpMethod::Get, "/".to_string())
        }
    }

    fn parse_query_params(route: &str) -> HashMap<String, String> {
        let split = route.split('?').collect::<Vec<&str>>();
        if split.len() < 2 {
            HashMap::<String, String>::new()
        } else {
            let params_string = split[1];
            let params =
                params_string
                    .split('&')
                    .fold(HashMap::<String, String>::new(), |mut acc, cur| {
                        let split = cur.split(':').collect::<Vec<&str>>();
                        if let (Some(k), Some(v)) = (split.first(), split.last()) {
                            acc.insert(k.to_string(), v.to_string());
                        }
                        acc
                    });
            params
        }
    }

    fn parse_headers(lines: &mut Lines) -> HashMap<String, String> {
        let mut headers = HashMap::<String, String>::new();

        while let Some(line) = &lines.next() {
            if line == &"" {
                break;
            } else {
                let split: Vec<&str> = line.split(":").map(|s| s.trim_end().trim_start()).collect();
                if let (Some(key), Some(value)) = (split.first(), split.last()) {
                    headers.insert(key.to_string(), value.to_string());
                }
            }
        }
        headers
    }
}
