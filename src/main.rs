use std::env;
mod curl;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];

    curl::parse_input(file);
}
