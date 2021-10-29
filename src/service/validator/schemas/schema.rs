use regex::Regex;
use valico::json_dsl::Builder;

pub trait Schema {
    fn get_schema() -> Builder;
}

pub fn get_positive_integer_regex() -> Regex {
    Regex::new(r"^([1-9]\d*)$").unwrap()
}

pub fn get_hex_color_regex() -> Regex {
    Regex::new(r"^#([A-Fa-f0-9]{8}|[A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$").unwrap()
}
