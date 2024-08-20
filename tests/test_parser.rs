extern crate casio_parser;
use casio_parser::parser::{
    self,
    parser::{GShockRecord, TimepieceRecord, WatchRecord},
    tokensier::Tokens,
};

#[test]
fn test_parser_parses_timepiece() {
    let timepiece_code = "AQ-230AD-8EV";
    let Tokens { tokens, watch_type } =
        parser::tokensier::tokeniser(&timepiece_code).expect("Failed to tokenise");

    let model_info = parser::parser::parse_record(&watch_type, tokens);

    let expected_model_info = TimepieceRecord {
        category: "Ana-Digi",
        band: "Stainless Steel",
        colour: "Silver",
        dial: "Analog Sticks",
    };

    match model_info {
        Some(WatchRecord::Timepiece(actual_model_info)) => {
            assert_eq!(actual_model_info, expected_model_info);
        }
        _ => panic!("Failed to parse record"),
    }
}

#[test]
fn test_parser_parses_gshock() {
    let gshock_code = "GA-100C-1A3ER";
    let Tokens { tokens, watch_type } =
        parser::tokensier::tokeniser(&gshock_code).expect("Failed to tokenise");

    let model_info = parser::parser::parse_record(&watch_type, tokens);

    let expected_model_info = GShockRecord {
        category: "G-Shock Analogue",
        prefix: "None",
        suffix: "Classic",
        main_colour: "Black",
        accent_colour: "Green",
        order: "1st Wave",
        country: "Europe",
    };

    match model_info {
        Some(WatchRecord::Gshock(actual_model_info)) => {
            assert_eq!(actual_model_info, expected_model_info);
        }
        _ => panic!("Failed to parse record"),
    }
}
