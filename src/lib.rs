use crate::response::{GeminiResponse, Response};
use native_tls::TlsConnector;
use std::io;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

pub mod response;

pub fn make_request(url: url::Url) -> Result<GeminiResponse, Box<dyn std::error::Error + 'static>> {
    match url.scheme() {
        "gemini" => {
            let bytes = make_request_(url)?;
            match Response::try_from(bytes.as_slice())? {
                Response::Prompt(prompt) => Ok(GeminiResponse::from(Response::Prompt(prompt))),
                Response::Success(success) => Ok(GeminiResponse::from(Response::Success(success))),
                Response::Redirect(redirect) => {
                    Ok(GeminiResponse::from(Response::Redirect(redirect)))
                }
                Response::Failure(failure) => Ok(GeminiResponse::from(Response::Failure(failure))),
                Response::Certificate(certificate) => {
                    Ok(GeminiResponse::from(Response::Certificate(certificate)))
                }
            }
        }
        _ => Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Unsupported scheme: {}", url.scheme()),
        ))),
    }
}
fn make_request_(url: url::Url) -> Result<Vec<u8>, Box<dyn std::error::Error + 'static>> {
    const PORT: u16 = 1965;
    let hostname = url.host_str().ok_or("localhost")?;
    let addr = format!("{}:{}", hostname, PORT);
    let socket_addr = addr.to_socket_addrs()?.next().unwrap();

    let mut builder = TlsConnector::builder();
    builder.danger_accept_invalid_hostnames(true);
    builder.danger_accept_invalid_certs(true);
    let connector = builder.build()?;

    let tcp_stream = TcpStream::connect_timeout(&socket_addr, Duration::from_secs(5))?;
    let mut tls_stream = connector.connect(hostname, tcp_stream)?;

    tls_stream.write_all(format!("{}\r\n", url).as_bytes())?;
    let mut bucket = vec![];
    tls_stream.read_to_end(&mut bucket)?;
    Ok(bucket)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::*;
    #[test]
    fn test_fun_server() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let url = url::Url::parse("gemini://localhost")?;
        let GeminiResponse {
            prompt,
            success,
            redirect,
            failure,
            certificate,
        } = make_request(url)?;
        if prompt.is_some() {
            panic!("Unexpected prompt response");
        }
        if redirect.is_some() {
            panic!("Unexpected redirect response");
        }
        if failure.is_some() {
            panic!("Unexpected failure response");
        }
        if certificate.is_some() {
            panic!("Unexpected certificate response");
        }
        if let Some(success) = success {
            println!(
                "code: {}; mime type: {}; body: {}",
                success.code(),
                success.mime_type(),
                String::from_utf8_lossy(success.body())
            );
            assert_eq!("20 text/gemini;charset=utf-8;lang=en", success.status());
            assert_eq!("20", success.code());
            assert_eq!("text/gemini;charset=utf-8;lang=en", success.mime_type());
            assert!(!success.body().is_empty());
        } else {
            panic!("Not a success response");
        }
        Ok(())
    }
    #[test]
    fn test_read_gemini_not_found() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let url = url::Url::parse("gemini://otrn.org/presumably%20bogus")?;
        let GeminiResponse {
            prompt,
            success,
            redirect,
            failure,
            certificate,
        } = make_request(url)?;
        if prompt.is_some() {
            panic!("Unexpected prompt response");
        }
        if redirect.is_some() {
            panic!("Unexpected redirect response");
        }
        if success.is_some() {
            panic!("Unexpected success response");
        }
        if certificate.is_some() {
            panic!("Unexpected certificate response");
        }
        if let Some(failure) = failure {
            assert_eq!("51 Not found", failure.status());
            assert_eq!("51", failure.code());
            assert_eq!("Not found", failure.message());
        } else {
            panic!("Not a failure response");
        }
        Ok(())
    }
    #[test]
    fn test_read_gemini_permanent_redirect() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let url = url::Url::parse("gemini://gemini.ucant.org/gemlog/")?;
        let GeminiResponse {
            prompt,
            success,
            redirect,
            failure,
            certificate,
        } = make_request(url)?;
        if prompt.is_some() {
            panic!("Unexpected prompt response");
        }
        if success.is_some() {
            panic!("Unexpected success response");
        }
        if failure.is_some() {
            panic!("Unexpected failure response");
        }
        if certificate.is_some() {
            panic!("Unexpected certificate response");
        }
        if let Some(redirect) = redirect {
            // println!("code: {}; message: {}", redirect.code(), redirect.url());
            assert_eq!(
                "31 gemini://gemini.ucant.org:1965/gemlog/index.gemini",
                redirect.status()
            );
            assert_eq!("31", redirect.code());
            assert_eq!(
                "gemini://gemini.ucant.org:1965/gemlog/index.gemini",
                redirect.url()
            );
        } else {
            panic!("Not a redirect response");
        }
        Ok(())
    }
    #[test]
    fn test_read_gemini_prompt() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let url = url::Url::parse("gemini://kennedy.gemi.dev/search")?;
        let GeminiResponse {
            prompt,
            success,
            redirect,
            failure,
            certificate,
        } = make_request(url)?;
        if redirect.is_some() {
            panic!("Unexpected redirect response");
        }
        if success.is_some() {
            panic!("Unexpected success response");
        }
        if failure.is_some() {
            panic!("Unexpected failure response");
        }
        if certificate.is_some() {
            panic!("Unexpected certificate response");
        }
        if let Some(prompt) = prompt {
            // println!("code: {}; prompt: {}", prompt.code(), prompt.prompt());
            assert_eq!("10 Enter search query", prompt.status());
            assert_eq!("10", prompt.code());
            assert_eq!("Enter search query", prompt.prompt());
        } else {
            panic!("Not a prompt response");
        }
        Ok(())
    }
    #[test]
    #[should_panic]
    fn test_read_gemini_invalid_scheme() {
        if let Ok(url) = url::Url::parse("https://example.com") {
            let raw_response = make_request(url).expect("This should panic");
            println!("{:?}", raw_response);
        }
    }
}
