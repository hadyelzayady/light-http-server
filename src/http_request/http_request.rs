use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
    str::FromStr,
};

use crate::common::{http_headers::HttpRequestHeaders, http_version::HttpVersion};

use super::http_method::HttpMethod;

#[derive(Debug)]
pub struct HttpRequest {
    method: HttpMethod,
    uri: String,
    version: HttpVersion,
    headers: Option<HttpRequestHeaders>,
    body: Option<Vec<String>>,
}

impl HttpRequest {
    pub fn from_reader(reader: &mut BufReader<&mut TcpStream>) -> Result<Self, ()> {
        let mut buf = String::new();
        let read_line = reader.read_line(&mut buf);
        match read_line {
            Ok(_v) => println!("read line: {buf}"),
            Err(_) => return Err(()),
        }
        let request_line = buf.split(' ').collect::<Vec<&str>>();
        if request_line.len() != 3 {
            panic!("Invalid request line")
        }
        let method = HttpMethod::from_str(request_line.first().unwrap())?;
        let url = request_line.get(1).unwrap();
        println!("request line {:?}", request_line);
        let version = HttpVersion::from_str(request_line.get(2).unwrap().trim_end()).unwrap();
        let request = HttpRequest {
            method,
            uri: url.to_string(),
            version,
            headers: None,
            body: None,
        };
        Ok(request)
    }
}
