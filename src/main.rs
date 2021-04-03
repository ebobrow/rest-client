use std::env;
mod request;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];

    request::parse_input(file);
}
