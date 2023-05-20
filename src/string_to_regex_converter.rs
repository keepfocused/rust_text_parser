use regex::Regex;

pub fn build_regex(pattern: &str) -> Result<Regex, regex::Error> {
    Regex::new(pattern)
}