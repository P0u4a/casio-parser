extern crate casio_parser;
use casio_parser::parser::{self, tokensier::Tokens};
use std::vec;

#[test]
fn test_tokensier_identifies_timepiece() {
    let timepiece_code = "FT-610WV-3BV";
    let Tokens { watch_type, .. } =
        parser::tokensier::tokeniser(&timepiece_code).expect("Failed to tokenise");
    const TIMEPIECE: &str = "timepiece";
    assert_eq!(watch_type, TIMEPIECE);
}

#[test]
fn test_tokeniser_identifies_gshock() {
    let gshock_code = "GW-B5600BC-1BER";
    let Tokens { watch_type, .. } =
        parser::tokensier::tokeniser(&gshock_code).expect("Failed to tokenise");

    const GSHOCK: &str = "gshock";
    assert_eq!(watch_type, GSHOCK);
}

#[test]
fn test_tokeniser_tokenises_timepiece() {
    let timepiece_code = "FT-610WV-3BV";
    let Tokens { tokens, .. } =
        parser::tokensier::tokeniser(&timepiece_code).expect("Failed to tokenise");

    let expected_tokens: Vec<&str> = vec!["FT", "610W", "V", "3", "BV"];
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokeniser_tokenises_gshock() {
    let gshock_code_no_prefix = "GA-100C-1A3ER";
    let Tokens { tokens, .. } =
        parser::tokensier::tokeniser(&gshock_code_no_prefix).expect("Failed to tokenise");

    let expected_tokens: Vec<&str> = vec!["GA", "", "100", "C", "1", "A", "3", "ER"];
    assert_eq!(tokens, expected_tokens);

    let gshock_code_no_accent = "GW-B5600BC-1BER";
    let Tokens { tokens, .. } =
        parser::tokensier::tokeniser(&gshock_code_no_accent).expect("Failed to tokenise");

    let expected_tokens: Vec<&str> = vec!["GW", "B", "5600", "BC", "1", "B", "", "ER"];
    assert_eq!(tokens, expected_tokens);
}
