pub mod cli;

use ansi_term::Colour::*;
use colored_json::prelude::*;
use colored_json::{Color, Styler};
use reqwest::blocking::{Client, RequestBuilder};
use std::fs;

// options not supported by reqwest
const METHODS: [&str; 6] = ["GET", "HEAD", "POST", "PUT", "DELETE", "PATCH"];

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

    fn error(&self, line: usize, msg: &str) -> String {
        format!("Error (line {}): {}", line, msg)
    }

    fn get_uri(&self) -> Result<String, String> {
        let mut host = self.lines[0].text.to_string();
        let last_line = &self.lines[self.lines.len() - 1];
        let mut words = last_line.text.split(' ');
        words.next();
        let location = match words.next() {
            Some(location) if !location.is_empty() => location,
            _ => {
                return Err(self.error(last_line.number, "Expected location"));
            }
        };
        host.push_str(&location);
        Ok(host)
    }

    fn parse(&self, client: &Client) -> Result<RequestBuilder, String> {
        let uri = match self.get_uri() {
            Ok(uri) => uri,
            Err(e) => return Err(e),
        };

        let mut in_body = false;
        let mut body = "".to_string();

        let mut headers: Vec<&Line> = vec![];

        for line in self.lines[1..self.lines.len() - 1].iter() {
            // Don't like this. Very hacky
            if in_body || line.text.starts_with('{') || line.text.contains('}') {
                body.push_str(&line.text);
                in_body = !line.text.ends_with('}');
            } else {
                // Assume headers
                headers.push(line);
            }
        }

        let mut req: RequestBuilder = match &self.method[..] {
            "GET" => client.get(&uri),
            "POST" => client.post(&uri),
            "PUT" => client.put(&uri),
            "DELETE" => client.delete(&uri),
            "HEAD" => client.head(&uri),
            "PATCH" => client.patch(&uri),
            //"OPTIONS" => client.options(uri), // Uh oh
            method => {
                return Err(
                    self.error(self.lines[0].number, &format!("Invalid method: {}", method))
                );
            }
        };

        for header in headers {
            let mut parts = header.text.split(':');
            let name = match parts.next() {
                Some(name) => name,
                _ => {
                    return Err(self.error(header.number, "Invalid header syntax"));
                }
            };
            let value = match parts.next() {
                Some(name) => name.trim(),
                _ => {
                    return Err(self.error(header.number, "Invalid header syntax"));
                }
            };

            req = req.header(name, value);
        }

        req = match &body[..] {
            "" => req,
            _ => req.body(body),
        };

        println!("{} {}\n", self.method, Yellow.paint(uri));

        Ok(req)
    }
}

pub fn run(config: cli::Cli) {
    #[cfg(windows)]
    let enabled = ansi_term::enable_ansi_support();

    let contents = fs::read_to_string(config.path).expect("Something went wrong reading the file");
    let client = Client::new();

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
                println!("\n---------------");
                let req = Request::new(lines[start_line..n].to_vec(), method).parse(&client);
                match req {
                    Ok(req) => {
                        send_req(req, config.concise).unwrap_or_else(|e| {
                            println!("{}", e);
                        });
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }

                start_line = n;
            }
        }
    }
}

fn send_req(req: RequestBuilder, concise: bool) -> Result<(), reqwest::Error> {
    let res = match req.send() {
        Ok(res) => res,
        Err(e) => {
            return Err(e);
        }
    };

    let status = res.status();
    let reason = match status.canonical_reason() {
        Some(reason) => reason,
        None => "",
    };
    let code = status.as_str();
    let code = if status.is_success() {
        Green.paint(code)
    } else if status.is_redirection() {
        Blue.paint(code)
    } else if status.is_informational() {
        Yellow.paint(code)
    } else {
        Red.paint(code)
    };
    println!("{} {}", code, reason);

    if !concise {
        for (key, value) in res.headers().iter() {
            println!("{}: {:?}", Cyan.paint(key.as_str()), value);
        }
        println!();
    }

    let default = &reqwest::header::HeaderValue::from_str("").unwrap();
    let content_type = res.headers().get("content-type").unwrap_or(default);
    if content_type == "application/json; charset=utf-8" {
        let res_body = res
            .text()
            .unwrap()
            .to_colored_json_with_styler(
                ColorMode::default().eval(),
                Styler {
                    key: Color::Green.normal(),
                    string_value: Color::Cyan.normal(),
                    integer_value: Color::Yellow.normal(),
                    float_value: Color::Yellow.normal(),
                    object_brackets: Default::default(),
                    array_brackets: Default::default(),
                    bool_value: Color::Red.normal(),
                    ..Default::default()
                },
            )
            .unwrap();
        println!("{:#}", res_body);
    } else {
        println!("{}", res.text().unwrap());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(text: &str, method: &str) -> Result<RequestBuilder, String> {
        let mut n = 0;
        let mut lines = vec![];
        for line in text.split("\n") {
            n += 1;
            lines.push(Line::new(line, n));
        }
        let client = Client::new();
        Request::new(lines, method).parse(&client)
    }

    #[test]
    fn get() {
        let req = setup("https://example.com\nGET /route", "GET")
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(req.method().as_str(), "GET");
        assert_eq!(
            *req.url(),
            reqwest::Url::parse("https://example.com/route").unwrap()
        );
        assert!(req.headers().is_empty());
        match req.body() {
            Some(_) => panic!("Body should be empty"),
            None => {} // OK
        }
    }

    #[test]
    fn no_location_specified() {
        let req = setup("http://localhost:8080\nPUT ", "PUT");
        match req {
            Ok(_) => {
                panic!("Expected error");
            }
            Err(e) => {
                assert_eq!(e, "Error (line 2): Expected location");
            }
        }
    }

    #[test]
    fn post() {
        let text = r#"http://localhost
Content-Type: application/json
{
    "key": "value"
}
POST /"#;
        let req = setup(text, "POST").unwrap().build().unwrap();

        assert_eq!(req.method().as_str(), "POST");
        assert_eq!(
            *req.url(),
            reqwest::Url::parse("http://localhost/").unwrap()
        );
        let headers = req.headers();
        assert_eq!(headers.len(), 1);
        assert_eq!(headers.get("content-type").unwrap(), &"application/json");
        match req.body() {
            Some(body) => {
                let expected_body = r#"{    "key": "value"}"#;
                assert_eq!(
                    body.as_bytes().unwrap(),
                    reqwest::blocking::Body::from(expected_body)
                        .as_bytes()
                        .unwrap()
                );
            }
            None => panic!("Expected body"),
        }
    }

    #[test]
    fn invalid_header() {
        let text = "https://www.example.com
Content-Type: text/html
other header
DELETE /api/thing";
        let req = setup(text, "DELETE");
        match req {
            Ok(_) => {
                panic!("Expected error");
            }
            Err(e) => {
                assert_eq!(e, "Error (line 3): Invalid header syntax");
            }
        }
    }
}
