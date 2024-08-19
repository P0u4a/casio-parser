use phf::phf_map;

/*
* TimePiece Mappings
*/

pub static CATEGORY_CODE: phf::Map<&'static str, &'static str> = phf_map! {
    "A" => "WR Digital",
    "F" => "WR Digital",
    "LA" => "WR Digital",
    "ABX" => "Twincept",
    "AQ" => "Ana-Digi",
    "AW" => "Analogue Water Resistant",
    "AWE" => "Ana-Digi",
    "BG" => "Baby-G",
    "MSG" => "Baby-G",
    "DB" => "Databank / With Calc.",
    "DBE" => "Databank / With Calc.",
    "EDB" => "Databank / With Calc.",
    "DBC" => "Databank / With Calc.",
    "G" => "G-Shock",
    "GW" => "G-Shock",
    "MTG" => "G-Shock",
    "EF" => "Edifice Analog",
    "EFA" => "Edifice Ana-Digi",
    "FT" => "Forester Mens",
    "FTL" => "Forester Ladies",
    "MTD" => "Marine Gear Mens",
    "LTD" => "Marine Gear Ladies",
    "MDAS" => "Solar Ana-Digi",
    "MTP" => "Analog Mens",
    "LTP" => "Analog Ladies",
    "MQ" => "WR analog",
    "LQ" => "WR analog",
    "OC" => "Oceanus",
    "OCW" => "Oceanus",
    "PAT" => "Pathfinder",
    "PAG" => "Pathfinder",
    "PAW" => "Pathfinder",
    "PRG" => "Pathfinder",
    "SHN" => "Sheen",
    "SPF" => "Sea-Pathfinder",
    "SPS" => "Sea-Pathfinder",
    "SPW" => "Sea-Pathfinder",
    "TRT" => "Twin Resist",
    "W" => "50M Sport",
    "LW" => "50M Sport",
    "WE" => "50M Sport",
    "WV" => "Waveceptor",
    "WVA" => "Waveceptor",
};

pub static BAND_CODE: phf::Map<&'static str, &'static str> = phf_map! {
    "D" => "Stainless Steel",
    "A" => "Stainless Steel",
    "G" => "Two Tone",
    "E" => "Expansion",
    "L" => "Leather",
    "B" => "Nylon",
    "F" => "Nylon",
    "V" => "Velcro"
};

pub static DIAL_CODE: phf::Map<&'static str, &'static str> = phf_map! {
    "V" => "Digital",
    "EV" => "Analog Sticks",
    "BV" => "Analog Numerals",
    "AV" => "Metal",
    "C" => "Calculator"
};
