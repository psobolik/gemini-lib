use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LineType {
    PreformatToggle,
    Heading1,
    Heading2,
    Heading3,
    Link,
    ListItem,
    Quote,
    Text,
}
pub struct GeminiLine {
    line_type: LineType,
    line_text: Option<String>,
}
impl GeminiLine {
    pub fn new(line_type: LineType, line_text: Option<String>) -> Self {
        Self { line_type, line_text }
    }
    pub fn line_type(&self) -> LineType {self.line_type}
    pub fn line_text(&self) -> Option<String> {self.line_text.clone()}
}

impl FromStr for GeminiLine {
    type Err = u8; // No error is possible

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if line.starts_with("```") {
            Ok(GeminiLine::new(LineType::PreformatToggle, None))
        } else if let Some(line_text) = line.strip_prefix("###") {
            Ok(GeminiLine::new(LineType::Heading3, Some(line_text.to_string())))
        } else if let Some(line_text) = line.strip_prefix("##") {
            Ok(GeminiLine::new(LineType::Heading2, Some(line_text.to_string())))
        } else if let Some(line_text) = line.strip_prefix("#") {
            Ok(GeminiLine::new(LineType::Heading1, Some(line_text.to_string())))
        } else if let Some(line_text) = line.strip_prefix("=>") {
            Ok(GeminiLine::new(LineType::Link, Some(line_text.to_string())))
        } else if let Some(line_text) = line.strip_prefix("*") {
            Ok(GeminiLine::new(LineType::ListItem, Some(line_text.to_string())))
        } else if let Some(line_text) = line.strip_prefix(">") {
            Ok(GeminiLine::new(LineType::Quote, Some(line_text.to_string())))
        } else {
            Ok(GeminiLine::new(LineType::Text, Some(line.to_string())))
        }
    }
}

