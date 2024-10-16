use crate::response::Response;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Prompt {
    status: String,
    code: String,
    prompt: String,
}

impl Prompt {
    pub fn new(status: &str, code: &str, prompt: &str) -> Self {
        Self {
            status: status.to_string(),
            code: code.to_string(),
            prompt: prompt.to_string(),
        }
    }
    pub fn status(&self) -> &str {
        &self.status
    }
    pub fn code(&self) -> &str {
        &self.code
    }
    pub fn prompt(&self) -> &str {
        &self.prompt
    }
    pub fn is_sensitive(&self) -> bool {
        !self.code.starts_with("11")
    }
}

impl From<String> for Prompt {
    fn from(status: String) -> Self {
        let (code, prompt) = Response::split_status(&status);
        Self::new(status.as_str(), code, prompt)
    }
}
