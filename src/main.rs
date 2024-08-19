mod mappings;
mod parser;
use std::env;

fn main() {
    // TODO print model info in a more user-friendly format
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please pass a watch code like this: cargo run -- <watch-code>");
    }

    let (watch_type, tokens) = parser::tokensier::tokeniser(&args[1]);

    let model_info = parser::parser::parse_record(&watch_type, tokens);
    if let Some(value) = model_info {
        println!("{:?}", value);
    } else {
        eprintln!("Error parsing watch code");
    }
}
