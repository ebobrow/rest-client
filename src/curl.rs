use std::io::Write;
use std::process::Command;
use std::{fs, io};

struct Request {
    text: String,
    method: String,
    // TODO: This is not accurate b/c comments and blank lines are filtered out
    first_line: u32,
    last_line: u32,
}

const METHODS: [&str; 7] = ["GET", "HEAD", "OPTIONS", "POST", "PUT", "DELETE", "PATCH"];

impl Request {
    fn new(text: &str, method: &str, first_line: u32, last_line: u32) -> Request {
        Request {
            text: text.to_string(),
            method: method.to_string(),
            first_line,
            last_line,
        }
    }

    fn error(&self, line: u32, msg: &str) {
        println!("Error (line {}): {}", line, msg);
    }

    fn parse(self) {
        let mut lines = self.text.lines();
        let base_url = lines.next();
        let mut base_url = match base_url {
            Some(u) => u.to_string(),
            None => {
                self.error(self.first_line, "Expected URL");
                return;
            }
        };

        let mut cur_line = self.first_line;

        let mut in_body = false;
        let mut body = "".to_string();

        let mut headers: Vec<&str> = vec![];

        for line in lines {
            cur_line += 1;
            if cur_line == self.last_line {
                let mut words = line.split(' ');
                words.next();
                let url = match words.next() {
                    Some(u) => u,
                    None => {
                        self.error(self.last_line, "Expected URL");
                        ""
                    }
                };
                base_url.push_str(url);
            } else if in_body || line.starts_with('{') {
                body.push_str(line);
                // TODO: Nested json, like:
                // {
                // "value": {
                // "inner_value": "sfgsdfg"
                // } <- Would trigger end
                // }
                in_body = !line.ends_with('}');
            } else {
                // Assume headers
                headers.push(line);
            }
        }

        let mut args: Vec<&str> = vec!["-X", &self.method];
        for header in headers {
            args.push("-H");
            args.push(header);
        }
        if body != "" {
            args.push("-d");
            args.push(&body);
        }
        args.push(&base_url);
        args.push("-i");

        println!("{} {}\n", self.method, base_url);

        let output = Command::new("curl")
            .args(&args)
            .output()
            .expect("Something went wrong");
        io::stdout().write_all(&output.stdout).unwrap();
    }
}

pub fn parse_input(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut cur_idx = 0;
    let mut start_idx = 0;
    let mut cur_line = 0;
    let mut start_line = 1;

    let filtered: String = contents
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| format!("{}\n", line)) // TODO: this feels hacky
        .collect();

    for line in filtered.lines() {
        cur_idx += line.len() + 1;
        cur_line += 1;
        for method in METHODS.iter() {
            if line.starts_with(method) {
                println!("\n---------------\n");
                Request::new(&filtered[start_idx..cur_idx], method, start_line, cur_line).parse();
                start_idx = cur_idx;
                start_line = cur_line + 1;
            }
        }
    }
}
