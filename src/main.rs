mod mappings;
mod parser;
use parser::parser::WatchRecord;
use parser::parser::WatchRecordTrait;
use std::env;

fn handle_model_info(model_info: Option<WatchRecord>) {
    if let Some(res) = model_info {
        res.pretty_print();
    } else {
        eprintln!("Failed to parse watch code. A symbol did not exist in the available mappings.");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please pass a watch code like this: cargo run -- <watch-code>");
    }

    match parser::tokensier::tokeniser(&args[1]) {
        Ok(res) => {
            let model_info = parser::parser::parse_record(res.watch_type, res.tokens);
            handle_model_info(model_info);
        }

        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
