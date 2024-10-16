use crate::response::Response;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Failure {
    status: String,
    code: String,
    message: String,
}

impl Failure {
    pub fn new(status: &str, code: &str, message: &str) -> Self {
        Self {
            status: status.to_string(),
            code: code.to_string(),
            message: message.to_string(),
        }
    }
    pub fn status(&self) -> &str {
        &self.status
    }
    pub fn code(&self) -> &str {
        &self.code
    }
    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn is_permanent(&self) -> bool {
        !self.code.starts_with('4')
    }
}

impl From<String> for Failure {
    fn from(status: String) -> Self {
        let (code, message) = Response::split_status(&status);
        Self::new(status.as_str(), code, message)
    }
}
