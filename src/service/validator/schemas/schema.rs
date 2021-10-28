use regex::Regex;
use valico::json_dsl::Builder;

pub trait Schema {
    fn get_schema() -> Builder;
}

pub fn get_positive_integer_regex() -> Regex {
    Regex::new(r"^([1-9]\d*)$").unwrap()
}
