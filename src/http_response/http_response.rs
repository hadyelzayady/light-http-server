use std::{io::Write, net::TcpStream};

use crate::common::{
    http_headers::HttpRequestHeaders, http_version::HttpVersion, status::HttpStatus,
};

#[derive(Debug)]
pub struct HttpResponse<'a> {
    version: HttpVersion,
    stream: &'a TcpStream,
    status: Option<HttpStatus>,
    headers: Option<HttpRequestHeaders>,
    body: Option<String>,
}

impl<'a> HttpResponse<'a> {
    pub fn new(stream: &'a TcpStream) -> Self {
        HttpResponse {
            stream,
            status: Option::default(),
            headers: Option::default(),
            body: Option::default(),
            version: HttpVersion::HTTP1_1,
        }
    }

    pub fn set_status(&mut self, status: HttpStatus) {
        self.status = Some(status);
    }

    pub fn send(&mut self) {
        self.stream.write_all(self.to_string().as_bytes());
    }

    pub fn set_body(&mut self, body: String) {
        self.body = Some(body);
    }
    pub fn add_header(&mut self, key: String, val: String) {
        if self.headers.is_none() {
            self.headers = Some(HttpRequestHeaders::new());
        }
        self.headers
            .as_mut()
            .unwrap()
            .insert(key, val);
    }
}
fn get_headers(headers: &HttpRequestHeaders) -> String {
    let mut response = String::new();
    for (key, val) in headers.iter() {
        response.push_str(format!("{key}: {val}\r\n").as_str());
    }
    response
}

impl<'a> ToString for HttpResponse<'a> {
    // TODO: Implement Zero Copy instead to write directly into network card buffer
    fn to_string(&self) -> String {
        let mut response = String::new();
        response.push_str(&self.version.to_string());
        response.push(' ');
        response.push_str(&self.status.as_ref().unwrap().to_string());
        response.push_str("\r\n");
        if self.headers.is_some() {
            response.push_str(&get_headers(self.headers.as_ref().unwrap()));
        }
        response.push_str("\r\n");
        response.push_str(self.body.as_ref().unwrap());
        response
    }
}
