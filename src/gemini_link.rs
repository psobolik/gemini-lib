use std::str::FromStr;
use regex::Regex;

#[derive(Debug)]
pub struct GeminiLink {
    link: String,
    name: String,
}

impl GeminiLink {
    pub fn new(link: &str, name: &str) -> Self {
        Self { link: String::from(link), name: String::from(name) }
    }
}

impl FromStr for GeminiLink {
    type Err = u8; // No error is possible

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut link: String = String::default();
        let mut name: String = String::default();

        let regexp = Regex::new(r"(?<link>\S+)\s?(?<name>.*)").unwrap();
        if let Some(captures) = regexp.captures(value) {
            link = captures["link"].to_string();
            name = captures["name"].to_string();
        }
        Ok(GeminiLink::new(&link, &name))
    }
}