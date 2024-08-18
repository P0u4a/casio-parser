extern crate casio_parser;
use casio_parser::parser;
use std::vec;

#[test]
fn test_tokensier_identifies_timepiece() {
    let timepiece_code = "FT-610WV-3BV";
    let (watch_type, _) = parser::tokensier::tokeniser(&timepiece_code);

    const TIMEPIECE: &str = "Timepiece";
    assert_eq!(watch_type, TIMEPIECE);
}

#[test]
fn test_tokeniser_identifies_gshock() {
    let gshock_code = "GW-B5600BC-1BER";
    let (watch_type, _) = parser::tokensier::tokeniser(&gshock_code);

    const GSHOCK: &str = "G-Shock";
    assert_eq!(watch_type, GSHOCK);
}

#[test]
fn test_tokeniser_tokenises_timepiece() {
    let timepiece_code = "FT-610WV-3BV";
    let (_, tokens) = parser::tokensier::tokeniser(&timepiece_code);

    let expected_tokens: Vec<&str> = vec!["FT", "610W", "V", "3", "BV"];
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokeniser_tokenises_gshock() {
    let gshock_code_no_prefix = "GA-100C-1A3ER";
    let (_, tokens) = parser::tokensier::tokeniser(&gshock_code_no_prefix);

    let expected_tokens: Vec<&str> = vec!["GA", "", "100", "C", "1", "A", "3", "ER"];
    assert_eq!(tokens, expected_tokens);

    let gshock_code_no_accent = "GW-B5600BC-1BER";
    let (_, tokens) = parser::tokensier::tokeniser(&gshock_code_no_accent);

    let expected_tokens: Vec<&str> = vec!["GW", "B", "5600", "BC", "1", "B", "", "ER"];
    assert_eq!(tokens, expected_tokens);
}
