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
}

impl From<String> for Success {
    fn from(s: String) -> Self {
        let (code, mime_type) = Response::split_status(&s);
        Self::new(s.as_str(), code, mime_type)
    }
}
