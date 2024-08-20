use phf::phf_map;

/*
* Common Mappings
*/

pub static COLOUR_CODE: phf::Map<&'static str, &'static str> = phf_map! {
    "1" => "Black",
    "2" => "Blue",
    "3" => "Green",
    "4" => "Red",
    "5" => "Brown",
    "6" => "Purple",
    "7" => "White",
    "8" => "Silver",
    "9" => "Gold",
};
