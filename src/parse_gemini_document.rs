use crate::gemini_line::{GeminiLine, LineType};
use crate::gemini_link::GeminiLink;
use crate::response::Success;
use std::str::FromStr;

pub fn parse_gemini_document<F1, F2, F3, F4, F5, F6>(
    success: Success,
    preformatted_text: F1,
    link: F2,
    heading: F3,
    list_item: F4,
    quoted_text: F5,
    plain_text: F6,
) where
    F1: Fn(Vec<String>),
    F2: Fn(GeminiLink),
    F3: Fn((u8, String)),
    F4: Fn(String),
    F5: Fn(String),
    F6: Fn(String),
{
    if !success.is_gemini() { return; };

    let mut preformatted_lines: Vec<String> = vec!();
    let mut preformatting = false;

    success.lines().into_iter().for_each(|line| {
        let gemini_line = GeminiLine::from_str(&line).unwrap();
        if gemini_line.line_type() == LineType::PreformatToggle {
            // We're currently reading preformatted lines, so emit those lines
            if preformatting {
                preformatted_text(preformatted_lines.to_vec());
                preformatted_lines.clear();
            }
            // Toggle the preformat flag
            preformatting = !preformatting;
        } else if preformatting {
            preformatted_lines.push(line.to_string());
        } else {
            match gemini_line.line_type() {
                LineType::Heading1 => heading((1, gemini_line.line_text().unwrap_or_default().to_string())),
                LineType::Heading2 => heading((2, gemini_line.line_text().unwrap_or_default().to_string())),
                LineType::Heading3 => heading((3, gemini_line.line_text().unwrap_or_default().to_string())),
                LineType::Link => link(GeminiLink::from_str(gemini_line.line_text().unwrap_or_default().as_str()).unwrap()),
                LineType::ListItem => list_item(gemini_line.line_text().unwrap_or_default().to_string()),
                LineType::Quote => quoted_text(gemini_line.line_text().unwrap_or_default().to_string()),
                LineType::Text => plain_text(gemini_line.line_text().unwrap_or_default().to_string()),
                _ => {}
            }
        }
    })
}

