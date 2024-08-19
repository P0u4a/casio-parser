use crate::mappings;

#[derive(Debug, PartialEq)]
pub struct TimepieceRecord {
    pub category: &'static str,
    pub band: &'static str,
    pub colour: &'static str,
    pub dial: &'static str,
}

#[derive(Debug, PartialEq)]
pub struct GShockRecord {
    pub category: &'static str,
    pub prefix: &'static str,
    pub suffix: &'static str,
    pub main_colour: &'static str,
    pub accent_colour: &'static str,
    pub order: &'static str,
    pub country: &'static str,
}

pub fn parse_timepiece_record(codes: Vec<String>) -> Option<TimepieceRecord> {
    Some(TimepieceRecord {
        category: mappings::timepiece::CATEGORY_CODE.get(&codes[0])?,
        band: mappings::timepiece::BAND_CODE.get(&codes[2])?,
        colour: mappings::common::COLOUR_CODE.get(&codes[3])?,
        dial: mappings::timepiece::DIAL_CODE.get(&codes[4])?,
    })
}

pub fn parse_gshock_record(codes: Vec<String>) -> Option<GShockRecord> {
    Some(GShockRecord {
        category: mappings::gshock::TYPE_CODE.get(&codes[0])?,
        prefix: mappings::gshock::SERIES_PREFIX_CODE
            .get(&codes[1])
            .unwrap_or(&"None"),
        suffix: mappings::gshock::SERIES_SUFFIX_CODE.get(&codes[3])?,
        main_colour: mappings::common::COLOUR_CODE.get(&codes[4])?,
        order: mappings::gshock::ORDER_CODE.get(&codes[5])?,
        accent_colour: mappings::common::COLOUR_CODE
            .get(&codes[6])
            .unwrap_or(&"None"),
        country: mappings::gshock::COUNTRY_CODE.get(&codes[7])?,
    })
}

#[derive(Debug)]
pub enum WatchRecord {
    Gshock(GShockRecord),
    Timepiece(TimepieceRecord),
}

pub fn parse_record(record_type: &str, codes: Vec<String>) -> Option<WatchRecord> {
    match record_type {
        "gshock" => Some(WatchRecord::Gshock(parse_gshock_record(codes)?)),
        "timepiece" => Some(WatchRecord::Timepiece(parse_timepiece_record(codes)?)),
        _ => return None,
    }
}
