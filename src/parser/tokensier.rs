#[derive(Debug)]
pub struct Tokens {
    pub tokens: Vec<String>,
    pub watch_type: &'static str,
}

pub fn tokeniser(code: &str) -> Result<Tokens, &'static str> {
    /*
    *    Split the input into three sections delimited by hyphens

    *    Decide what the watch type and save this value as the tokenisation and mappings will be different
    *    depending on if the watch is a G-Shock or a vintage timepiece
    *
    *    For Timepiece:
    *    Get the category code which the first section element
    *    Read until we hit the first alphabetic character
    *    Save the series code, then get the band code and save it which is the next character
    *    Ignore the next hyphen
    *    The numeric value right after hyphen is the Colour code, and any string remaining is the Dial Code
    *
    *    For G-shock:
    *    Get the category code which the first section element
    *    Right after the hyphen, if the first letter is alphabetic, save the series prefix value otherwise None
    *    Then the next set of numbers until the next alphabetic character make up the series number
    *    The subsequent alphabetic characters up to the next hyphen are the Series Suffix
    *    Right after the second hyphen, there will be a single number which corresponds to colour, after this number
    *    there will be a letter which will be the order code
    *    After the order code, there may or may not be another digit representing the accent colour
    *    The remaining letters will be the country of the manufacturer
    *
    *    Disclaimer: Older models may not conform to these rules.
    **/

    let sections: Vec<&str> = code.split('-').collect();
    if sections.len() != 3 {
        return Err("Unexpected section length");
    }

    let watch_type = sections[0];

    if watch_type.contains("G") {
        return Ok(Tokens {
            tokens: tokenise_gshock(sections),
            watch_type: &"gshock",
        });
    }

    return Ok(Tokens {
        tokens: tokenenise_timepiece(sections),
        watch_type: "timepiece",
    });
}

fn tokenenise_timepiece(sections: Vec<&str>) -> Vec<String> {
    let category_code = sections[0];

    let series_and_band_code = sections[1];

    let mut series: String = String::new();
    let mut checkpoint = 0;

    for (i, c) in series_and_band_code.chars().enumerate() {
        if !c.is_digit(10) {
            checkpoint = i + 1;
            series.push(c);
            break;
        }

        series.push(c);
    }

    let band_code = &series_and_band_code[checkpoint..checkpoint + 1];

    let colour_and_dial_code = sections[2];
    let colour_code = &colour_and_dial_code[0..1];
    let dial_code = &colour_and_dial_code[1..];

    return vec![
        category_code.to_string(),
        series,
        band_code.to_string(),
        colour_code.to_string(),
        dial_code.to_string(),
    ];
}

fn tokenise_gshock(sections: Vec<&str>) -> Vec<String> {
    let category_code = sections[0];

    let prefix_and_suffix_code = sections[1];
    let mut prefix_and_suffix_code_iter = prefix_and_suffix_code.chars().peekable();

    let mut prefix_code = String::new();

    if prefix_and_suffix_code_iter
        .peek()
        .map_or(false, |c| c.is_alphabetic())
    {
        prefix_and_suffix_code_iter.next();
        prefix_code = prefix_and_suffix_code[0..1].to_string();
    }

    let mut series = String::new();
    let mut suffix_code = String::new();

    for c in prefix_and_suffix_code_iter {
        if c.is_ascii_alphabetic() {
            suffix_code.push(c);
            continue;
        }

        series.push(c as char);
    }

    let colours_and_country_code = sections[2];

    let main_colour_code = &colours_and_country_code[0..1];
    let order_code = &colours_and_country_code[1..2];
    let mut accent_colour_code = "";
    let country_code;

    if colours_and_country_code
        .chars()
        .nth(2)
        .map_or(false, |c| c.is_numeric())
    {
        accent_colour_code = &colours_and_country_code[2..3];
        country_code = &colours_and_country_code[3..];
    } else {
        country_code = &colours_and_country_code[2..];
    }

    return vec![
        category_code.to_string(),
        prefix_code,
        series,
        suffix_code,
        main_colour_code.to_string(),
        order_code.to_string(),
        accent_colour_code.to_string(),
        country_code.to_string(),
    ];
}
