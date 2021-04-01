use std::io::Write;
use std::process::Command;
use std::{fs, io};

struct Request {
    method: String,
    url: String,
    body: Option<String>,
}

impl Request {
    fn new(method: String, url: &str, body: Option<String>) -> Request {
        Request {
            method,
            url: url.to_string(),
            body,
        }
    }

    fn send(self) {
        let body_content: String; // TODO: Is this the best way to do this?
        let args = match self.body {
            Some(body) => {
                body_content = body;
                vec![
                    "-X",
                    &self.method,
                    "-H",
                    "Content-Type: application/json",
                    "-d",
                    &body_content,
                    &self.url,
                    "-i",
                ]
            }
            None => {
                vec!["-X", &self.method, &self.url, "-i"]
            }
        };
        let output = Command::new("curl")
            .args(&args)
            .output()
            .expect("Something went wrong");
        io::stdout().write_all(&output.stdout).unwrap();
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

        let method = words.next().expect("Invalid syntax: Method required");
        let url = words.next().expect("Invalid syntax: URL required");

        let body: String = lines.collect();
        let body = if body.is_empty() {
            println!();
            None
        } else {
            println!("Body: {}\n", body.trim());
            Some(body)
        };

        match method {
            "GET" | "HEAD" | "OPTIONS" => {
                Request::new(method.to_string(), url, None).send();
            }
            "POST" | "PUT" | "DELETE" | "PATCH" => {
                Request::new(method.to_string(), url, body).send();
            }
            _ => println!("Invalid method: {}", method),
        };
        println!();
    }
}
