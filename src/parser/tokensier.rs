// First split by hyphen
// Then find out timepiece or gshock
// parse based on the correct mappings

fn tokeniser(code: &str) {
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
}
