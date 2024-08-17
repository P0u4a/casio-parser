use phf::phf_map;

/*
* G-Shock Mappings
*/

pub static TYPE_CODE: phf::Map<&'static str, &'static str> = phf_map! {
"AW" => "Analogue Water Resistant",
"AWG" => "Analogue Water Resistant",
"DW " => "Digital Water Resistant",
"DWG"=> "Digital Water Resistant",
"DWM" => "Digital Water Resistant",
"DWX" => "Digital Water Resistant",
"G" => "G-Shock",
"GA "=> "G-Shock Analogue",
"GAS"=> "G-Shock Analogue",
"GAW"=> "G-Shock Analog Waveceptor",
"GD" => "G-Shock Digital",
"GDF" => "G-Shock Digital",
"GG" => "G-Shock Digital",
"GL "=> "Glide",
"GSL" => "Glide",
"GAX"=> "Glide",
"GLX" => "Glide Xtreme",
"GR" => "G-Shock GRAVITYMASTER",
"GRX" => "G-Shock Ride Xtreme",
"GS"=> "GIEZ",
"GSG"=> "GST",
"GW "=> "G-Shock Waveceptor",
"GWN" => "G-Shock Waveceptor Nautic",
"GX" => "G-Shock Xtreme",
"GXW" => "G-Shock Xtreme Waveceptor",
"MRG"=> "Majestic Reality G",
"MTG"=> "Metal Twisted G",
"WW "=>  "Wide (Temperature) Water Resist",
};

pub static SERIES_PREFIX_CODE: phf::Map<&'static str, &'static str> = phf_map! {
    "A" =>  "Analog",
    "B" => "Bluetooth",
    "G" => "GPS",
    "H" => "Hear Rate",
    "M" => "MultiBand",
    "Q" => "Quad Sensor",
    "S" => "Small",
    "T" => "Titanium",
    "X" => "X-large"
};

pub static SERIES_SUFFIX_CODE: phf::Map<&'static str, &'static str> = phf_map! {
    "AR" => "Adrenalin Red or Aurora Light",
    "BBM" => "Black Blue Matte",
    "BK" => "Real Black",
    "BY" => "Burning Sun",
    "CS" => "Crazy Colors",
    "CR" => "Coral Reef",
    "D" => "Stainless steel bracelet",
    "DG" => "Dagger",
    "DM" => "Denim",
    "GB" => "Black Gold",
    "HC" => "Hyper Colors, Hidden Coast",
    "K" => "ICERC",
    "KH" => "Master In Olive Drab Gravitymaster Aviation (Kandahar)",
    "JA" => "Jammin Color",
    "L" => "Leather watch strap",
    "LN" => "Layered Neon",
    "NP" => "Neo Punk",
    "NS" => "Night Surf",
    "MG" => "Metal Green",
    "MNT" => "Music Night Tokyo",
    "MMC" => "Black And Rose Gold",
    "MSA" => "Hawaii",
    "NB" => "Metallic colors",
    "NV" => "Navy",
    "PL" => "Polarized Color",
    "RB" => "Red Bull",
    "RD" => "Rescue Red",
    "SG" => "Skeleton Gold",
    "SK" => "Clear Skeleton",
    "SS" => "Sea Snake",
    "T" => "Titanium bracelet",
    "TAL" => "Autumn leave",
    "TC" => "Triple Crown Surfing",
    "TCB" => "Sakura Storm",
    "VB" => "Virtual Blue",
    "WC" => "World Cup edition",
    "WCCS" => "World Coral Reef Conservation Society",
};

pub enum Order {
    A,
    B,
    C,
    D,
    E,
}

pub static COUNTRY_CODE: phf::Map<&'static str, &'static str> = phf_map! {
    "CR" => "North / South America",
    "DR" => "Distant Region (Export)",
    "ER" => "Europe",
    "J" => "Japan",
    "JF" => "Japan",
    "JR" => "Japan Limited",
    "PF" => "China",
    "PFN" => "China",
    "PR" => "China",
    "PRT" => "China",
    "PRW" => "China",
    "V" => "Export",
    "VER" => "Export",
    "VDF" => "Australia / Oceania"
};
