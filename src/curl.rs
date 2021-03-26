use std::io::Write;
use std::process::Command;
use std::{fs, io};

enum Method {
    GET,
    POST,
    PUT,
    HEAD,
    DELETE,
    PATCH,
    OPTIONS,
}

struct Request {
    method: Method,
    url: String,
    body: Option<String>,
}

impl Request {
    fn new(method: Method, url: &str, body: Option<String>) -> Request {
        Request {
            method,
            url: url.to_string(),
            body,
        }
    }

    fn send(self) {
        let method = match self.method {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::HEAD => "HEAD",
            Method::DELETE => "DELETE",
            Method::PATCH => "PATCH",
            Method::OPTIONS => "OPTIONS",
        };

        match self.body {
            Some(body) => {
                let req = Command::new("curl")
                    .arg("-X")
                    .arg(method)
                    .arg("-H")
                    .arg("Content-Type: application/json")
                    .arg("-d")
                    .arg(body)
                    .arg(self.url)
                    .arg("-i")
                    .output()
                    .expect("Something went wrong");
                io::stdout().write_all(&req.stdout).unwrap();
            }
            None => {
                let req = Command::new("curl")
                    .arg("-X")
                    .arg(method)
                    .arg(self.url)
                    .arg("-i")
                    .output()
                    .expect("Something went wrong");
                io::stdout().write_all(&req.stdout).unwrap();
            }
        };
    }
}

pub fn parse_input(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let requests = contents.split("\n\n");
    for request in requests {
        handle_request(request);
    }
}

fn handle_request(request: &str) {
    let mut lines = request
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'));

    if let Some(line) = lines.next() {
        println!("\n---------------\n");
        println!("{}", line);
        let mut words = line.split(' ');

        let method = words.next().expect("Invalid syntax");
        let url = words.next().expect("Invalid syntax");

        let body: String = lines.collect();
        let body = if body.is_empty() {
            println!();
            None
        } else {
            println!("Body: {}", body.trim());
            Some(body)
        };

        match method {
            "GET" => {
                Request::new(Method::GET, url, None).send();
            }
            "POST" => {
                Request::new(Method::POST, url, body).send();
            }
            "PUT" => {
                Request::new(Method::PUT, url, body).send();
            }
            "HEAD" => {
                Request::new(Method::HEAD, url, None).send();
            }
            "DELETE" => {
                Request::new(Method::DELETE, url, body).send();
            }
            "PATCH" => {
                Request::new(Method::PATCH, url, body).send();
            }
            "OPTIONS" => {
                Request::new(Method::OPTIONS, url, None).send();
            }
            _ => println!("Invalid method"),
        };
        println!();
    }
}
