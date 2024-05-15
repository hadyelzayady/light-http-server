use light_http_server::{
    common::status::HttpStatus, http_request::http_request::HttpRequest,
    http_response::http_response::HttpResponse, ThreadPool,
};
use std::{
    io::BufReader,
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Listening On: http://127.0.0.1:7878");
    let pool = ThreadPool::build(2);
    // TODO thread pool with event loop
    for stream in listner.incoming() {
        let stream = stream.unwrap();
        pool.as_ref().unwrap().execute(|| {
            println!("{:?}", thread::current().id());
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let http_request = HttpRequest::from_reader(&mut buf_reader);
    let mut resp = HttpResponse::new(&stream);
    handle_http_request(&http_request.unwrap(), &mut resp);
}

fn handle_http_request(_http_request: &HttpRequest, res_handler: &mut HttpResponse) {
    res_handler.set_status(HttpStatus::OK);
    res_handler.set_body("Welcome To Light Http Server Built From scratch with standard library ( thread pool and event loop -> threadloop ;) ) By hadyelzayady".to_string());
    res_handler.add_header("server".to_string(), "ligh-app".to_string());
    res_handler.send();
}
