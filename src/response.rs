mod certificate;
mod failure;
pub mod gemini_response;
mod prompt;
mod redirect;
mod success;

pub use certificate::Certificate;
pub use failure::Failure;
pub use gemini_response::GeminiResponse;
pub use prompt::Prompt;
pub use redirect::Redirect;
use serde::Serialize;
use std::io;
pub use success::Success;

#[derive(Debug, Serialize)]
pub enum Response {
    Prompt(Prompt),
    Success(Success),
    Redirect(Redirect),
    Failure(Failure),
    Certificate(Certificate),
}

impl Response {
    pub fn split_status(status: &String) -> (&str, &str) {
        if let Some(tuple) = status.split_once(" ") {
            tuple
        } else {
            ("", status)
        }
    }
}

impl TryFrom<&[u8]> for Response {
    type Error = io::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if let Some(position) = bytes.windows(2).position(|window| window == [0x0d, 0x0a]) {
            let status = String::from_utf8_lossy(&bytes[..position]).to_string();
            match status.chars().nth(0) {
                Some('1') => Ok(Response::Prompt(Prompt::from(status))),
                Some('2') => Ok(Response::Success(
                    Success::from(status).set_body(&bytes[position + 2..]), // Skip the CRLF after the status
                )),
                Some('3') => Ok(Response::Redirect(Redirect::from(status))),
                Some('4') | Some('5') => Ok(Response::Failure(Failure::from(status))),
                Some('6') => Ok(Response::Certificate(Certificate::from(status))),
                _ => Err(io::Error::new(io::ErrorKind::Other, status)), // Unexpected status code
            }
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "invalid response"))
        }
    }
}
