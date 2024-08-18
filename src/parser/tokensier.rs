pub fn tokeniser(code: &str) -> (&'static str, Vec<String>) {
    // Read until first hyphen
    // Decide on category and save it in result
    // If category is Gshock switch parsing rules
    // For timepiece:
    // Read until we hit the first alphabetic character
    // Save the series code, then get the band code and save it which is the next character
    // Ignore the next hyphen
    // The numeric value right after hyphen is the Color code, and any string remaining is the Dial Code

    // For G-shock (a little more involved):
    // First look up the Category string again to get the gshock type according to the correct rules
    // Right after the hyphen, if the first letter is alphabetic, save the series prefix value otherwise null,
    // Then the next set of numbers until the next alphabetic character make up the series number
    // Strings up to the next hyphen are the Series Suffix
    // Right after the hyphen, there will be a single number which corresponds to colour, after this number
    // there will be either a letter or number (swapped around), which will be the order (letter) and accent colour (number)
    // The remaining strings will be the country of make

    // Disclaimer: Older models may not conform to these rules.

    let sections: Vec<&str> = code.split('-').collect();
    // TODO use Option Type
    if sections.len() != 3 {
        return ("", vec![]);
    }

    let watch_type = sections[0];

    if watch_type.contains("G") {
        return ("G-Shock", tokenise_gshock(sections));
    }

    return ("Timepiece", tokenenise_timepiece(sections));
}

fn tokenenise_timepiece(sections: Vec<&str>) -> Vec<String> {
    let category_code = sections[0];

    let series_and_band_code = sections[1];
    println!("{}", series_and_band_code);

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

// TODO make return types Option<> when appropriate
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
