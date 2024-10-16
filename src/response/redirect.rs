use crate::response::Response;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Redirect {
    status: String,
    code: String,
    url: String,
}

impl Redirect {
    pub fn new(status: &str, code: &str, url: &str) -> Self {
        Self {
            status: status.to_string(),
            code: code.to_string(),
            url: url.to_string(),
        }
    }
    pub fn status(&self) -> &str {
        &self.status
    }
    pub fn code(&self) -> &str {
        &self.code
    }
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn is_permanent(&self) -> bool {
        self.code.starts_with("31")
    }
}

impl From<String> for Redirect {
    fn from(status: String) -> Self {
        let (code, url) = Response::split_status(&status);
        Self::new(status.as_str(), code, url)
    }
}
