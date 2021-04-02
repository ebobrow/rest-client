use json;
use reqwest;
use std::fs;

const METHODS: [&str; 7] = ["GET", "HEAD", "OPTIONS", "POST", "PUT", "DELETE", "PATCH"];

#[derive(Clone)]
struct Line {
    text: String,
    number: usize,
}

impl Line {
    fn new(text: &str, number: usize) -> Line {
        Line {
            text: text.to_string(),
            number,
        }
    }
}

struct Request {
    lines: Vec<Line>,
    method: String,
}

impl Request {
    fn new(lines: Vec<Line>, method: &str) -> Request {
        Request {
            lines,
            method: method.to_string(),
        }
    }

    fn error(&self, line: usize, msg: &str) {
        println!("Error (line {}): {}", line, msg);
    }

    fn get_uri(&self) -> Result<String, String> {
        let mut host = self.lines[0].text.to_string();
        let last_line = &self.lines[self.lines.len() - 1];
        let mut words = last_line.text.split(' ');
        words.next();
        let location = match words.next() {
            Some(location) if !location.is_empty() => location,
            _ => {
                self.error(last_line.number, "Expected location");
                return Err("Invalid syntax".to_string());
            }
        };
        host.push_str(&location);
        Ok(host)
    }

    fn parse(&self) -> Result<(), String> {
        let uri = match self.get_uri() {
            Ok(uri) => uri,
            Err(e) => return Err(e),
        };

        let mut in_body = false;
        let mut body = "".to_string();

        let mut headers: Vec<&Line> = vec![];

        for line in self.lines[1..self.lines.len() - 1].iter() {
            if in_body || line.text.starts_with('{') {
                body.push_str(&line.text);
                // TODO: Nested json, like:
                // {
                // "value": {
                // "inner_value": "sfgsdfg"
                // } <- Would trigger end
                // }
                in_body = !line.text.ends_with('}');
            } else {
                // Assume headers
                headers.push(line);
            }
        }

        let client = reqwest::blocking::Client::new();

        let mut req: reqwest::blocking::RequestBuilder = match &self.method[..] {
            "GET" => client.get(&uri),
            "POST" => client.post(&uri),
            "PUT" => client.put(&uri),
            "DELETE" => client.delete(&uri),
            "HEAD" => client.head(&uri),
            "PATCH" => client.patch(&uri),
            //"OPTIONS" => client.options(uri), // Uh oh
            method => {
                self.error(self.lines[0].number, &format!("Invalid method: {}", method));
                return Err("".to_string());
            }
        };

        for header in headers {
            let mut parts = header.text.split(':');
            let name = match parts.next() {
                Some(name) => name,
                _ => {
                    self.error(header.number, "Invalid header syntax");
                    return Err("Syntax error".to_string());
                }
            };
            let value = match parts.next() {
                Some(name) => name,
                _ => {
                    self.error(header.number, "Invalid header syntax");
                    return Err("Syntax error".to_string());
                }
            };

            req = req.header(name, value);
        }

        match &body[..] {
            "" => {}
            _ => {
                req = req.body(body);
            }
        }

        println!("{} {}\n", self.method, uri);

        let res = req.send().unwrap();
        println!(
            "{} {}\n",
            res.status().as_u16(),
            res.status().canonical_reason().unwrap()
        );
        for (key, value) in res.headers().iter() {
            println!("{:?}: {:?}", key, value);
        }
        println!();
        if res.headers().get("content-type").unwrap() == "application/json; charset=utf-8" {
            let res_body = json::parse(&res.text().unwrap()).unwrap();
            println!("{:#}", res_body);
        } else {
            println!("{}", res.text().unwrap());
        }
        Ok(())
    }
}

pub fn parse_input(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut n = 0;
    let mut lines: Vec<Line> = vec![];

    for line in contents.lines() {
        n += 1;
        if !line.is_empty() && !line.starts_with('#') {
            lines.push(Line::new(line, n));
        }
    }

    let mut start_line = 0;
    n = 0;

    for line in &lines {
        n += 1;
        for method in METHODS.iter() {
            if line.text.starts_with(method) {
                println!("\n\n---------------");
                let _ = Request::new(lines[start_line..n].to_vec(), method).parse();
                start_line = n;
            }
        }
    }
}
