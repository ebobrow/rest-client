use std::io::Write;
use std::ops::Add;
use std::process::Command;
use std::{fs, io};

enum Methods {
    GET,
    POST,
}

struct Request {
    method: Methods,
    url: String,
    body: Option<String>,
}

impl Request {
    fn new(method: Methods, url: &str) -> Request {
        Request {
            method,
            url: url.to_owned(),
            body: None,
        }
    }

    fn update_body(mut self, to_add: String) {
        match self.body {
            Some(body) => self.body = Some(format!("{}\n{}", body, to_add)),
            None => self.body = Some(to_add),
        }
    }

    fn send(self) {
        match self.method {
            Methods::GET => {
                let req = Command::new("curl")
                    .arg(self.url)
                    .output()
                    .expect("Something went wrong");
                io::stdout().write_all(&req.stdout).unwrap();
            }
            _ => {}
        }
    }
}

pub fn parse_input(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut current_request: Option<Request> = None;
    for line in contents.lines() {
        match current_request {
            Some(ref mut request) => {
                // TODO
            }
            None => {
                if !line.is_empty() && !line.starts_with("#") {
                    let mut words = line.split(' ');
                    let method = words.next().expect("Invalid syntax");
                    let url = words.next().expect("Invalid syntax");

                    match method {
                        "GET" => {
                            println!("GET {}", url);
                            Request::new(Methods::GET, url).send();
                            println!("\n");
                        }
                        "POST" => {
                            current_request = Some(Request::new(Methods::POST, url));
                        }
                        _ => println!("Invalid syntax"),
                    }
                }
            }
        };
    }
}
