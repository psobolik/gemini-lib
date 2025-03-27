use crate::response::Response;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Success {
    status: String,
    code: String,
    mime_type: String,
    body: Vec<u8>,
}

impl Success {
    pub fn new(status: &str, code: &str, mime_type: &str) -> Self {
        Self {
            status: status.to_string(),
            code: code.to_string(),
            mime_type: mime_type.to_string(),
            body: Vec::new(),
        }
    }
    pub fn status(&self) -> &String {
        &self.status
    }
    pub fn code(&self) -> &String {
        &self.code
    }
    pub fn mime_type(&self) -> &String {
        &self.mime_type
    }
    pub fn body(&self) -> &Vec<u8> {
        &self.body
    }
    pub fn set_body(mut self, body: &[u8]) -> Self {
        self.body = body.to_vec();
        self
    }
    pub fn mime(&self) -> Option<mime::Mime> {
        match self.mime_type.parse::<mime::Mime>() {
            Ok(mime) => Some(mime),
            Err(_) => None,
        }
    }
    pub fn is_gemini(&self) -> bool {
        match self.mime() {
            Some(mime) => mime.type_() == "text" && mime.subtype().as_str() == "gemini",
            None => false,
        }
    }
    pub fn is_text_like(&self) -> bool {
        self.is_text() || self.is_xml()
    }
    fn test_mime_subtype(&self, test_subtype: &str) -> bool {
        match self.mime() {
            Some(mime) => mime.subtype().as_str() == test_subtype,
            None => false,
        }
    }
    fn test_mime_type(&self, test_type: &str) -> bool {
        match self.mime() {
            Some(mime) => {
                mime.type_() == test_type
            }
            _ => false,
        }
    }
    pub fn is_xml(&self) -> bool {
        self.test_mime_subtype("xml")
    }
    pub fn is_text(&self) -> bool {
        self.test_mime_type("text")
    }
    pub fn is_image(&self) -> bool {
        self.test_mime_type("image")
    }
    pub fn text(&self) -> String {
        String::from_utf8(self.body.clone()).unwrap_or_default()
    }
    pub fn lines(&self)-> Vec<String> {
        self.text().lines().map(|line| line.to_string()).collect()
    }
}

impl From<String> for Success {
    fn from(s: String) -> Self {
        let (code, mime_type) = Response::split_status(&s);
        Self::new(s.as_str(), code, mime_type)
    }
}
