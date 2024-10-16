use crate::response::{Certificate, Failure, Prompt, Redirect, Response, Success};
use serde::Serialize;

// TypeScript can't handle Rust enums, so we have this.
#[derive(Debug, Serialize)]
pub struct GeminiResponse {
    pub prompt: Option<Prompt>,
    pub success: Option<Success>,
    pub redirect: Option<Redirect>,
    pub failure: Option<Failure>,
    pub certificate: Option<Certificate>,
}

impl From<Response> for GeminiResponse {
    fn from(value: Response) -> Self {
        match value {
            Response::Prompt(prompt) => Self {
                prompt: Some(prompt),
                success: None,
                redirect: None,
                failure: None,
                certificate: None,
            },
            Response::Success(success) => Self {
                prompt: None,
                success: Some(success),
                redirect: None,
                failure: None,
                certificate: None,
            },
            Response::Redirect(redirect) => Self {
                prompt: None,
                success: None,
                redirect: Some(redirect),
                failure: None,
                certificate: None,
            },
            Response::Failure(failure) => Self {
                prompt: None,
                success: None,
                redirect: None,
                failure: Some(failure),
                certificate: None,
            },
            Response::Certificate(certificate) => Self {
                prompt: None,
                success: None,
                redirect: None,
                failure: None,
                certificate: Some(certificate),
            },
        }
    }
}
