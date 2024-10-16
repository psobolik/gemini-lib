use crate::response::Response;
use serde::Serialize;

pub enum CertificateStatus {
    Required,
    NotAuthorized,
    NotValid,
}

#[derive(Debug, Serialize)]
pub struct Certificate {
    status: String,
    code: String,
    message: String,
}

impl Certificate {
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
    pub fn certificate_status(&self) -> Option<CertificateStatus> {
        match self.code.as_str() {
            "60" => Some(CertificateStatus::Required),
            "61" => Some(CertificateStatus::NotAuthorized),
            "62" => Some(CertificateStatus::NotValid),
            _ => None, // This should never happen
        }
    }
}

impl From<String> for Certificate {
    fn from(status: String) -> Self {
        let (code, message) = Response::split_status(&status);
        Self::new(status.as_str(), code, message)
    }
}
