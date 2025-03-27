/*#[cfg(test)]
mod tests {
    use gemini_lib::make_request;
    use gemini_lib::response::Response::{Prompt, Redirect, Success, Failure};
    use gemini_lib::response::TaggedResponse;

    #[test]
    fn test_fun_server_prompt() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let url = url::Url::parse("gemini://localhost/prompt")?;
        let TaggedResponse { tag, response } = make_request(url)?;
        if tag != "prompt" {
            panic!("Not tagged as a Prompt Response: {:?}", response);
        }
        match response {
            Prompt(prompt) => {
                // println!("code: {}; prompt: {}", prompt.code(), prompt.prompt());
                assert_eq!("10 Enter search query", prompt.status());
                assert_eq!("10", prompt.code());
                assert_eq!("Enter search query", prompt.prompt());
            }
            _ => panic!("Unexpected Response: {:?}", response),
        }
        Ok(())
    }
    #[test]
    fn test_fun_server_password_prompt() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let url = url::Url::parse("gemini://localhost/password")?;
        let TaggedResponse { tag, response } = make_request(url)?;
        if tag != "prompt" {
            panic!("Not tagged as a Prompt Response: {:?}", response);
        }
        match response {
            Prompt(prompt) => {
                assert_eq!("11 Enter password", prompt.status());
                assert_eq!("11", prompt.code());
                assert_eq!("Enter password", prompt.prompt());
            }
            _ => panic!("Unexpected Response: {:?}", response),
        }
        Ok(())
    }
    #[test]
    fn test_fun_server_success() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let url = url::Url::parse("gemini://localhost/")?;
        let TaggedResponse { tag, response } = make_request(url)?;
        if tag != "success" {
            panic!("Not tagged as a Success Response: {:?}", response);
        }
        match response {
            Success(success) => {
                assert_eq!("20 text/gemini", success.status());
                assert_eq!("20", success.code());
                assert_eq!("text/gemini", success.mime_type());
                assert_eq!("It works\r\n".as_bytes(), success.body());
            }
            _ => panic!("Unexpected Response: {:?}", response),
        }
        Ok(())
    }
    #[test]
    fn test_fun_server_temporary_redirect() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let url = url::Url::parse("gemini://localhost/temporary%20redirect")?;
        let TaggedResponse { tag, response } = make_request(url)?;
        if tag != "redirect" {
            panic!("Not tagged as a Redirect Response: {:?}", response);
        }
        match response {
            Redirect(redirect) => {
                assert_eq!("30 gemini://localhost/sampler", redirect.status());
                assert_eq!("30", redirect.code());
                assert_eq!("gemini://localhost/sampler", redirect.url());
            }
            _ => panic!("Unexpected Response: {:?}", response),
        }
        Ok(())
    }
    #[test]
    fn test_fun_server_permanent_redirect() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let url = url::Url::parse("gemini://localhost/permanent%20redirect")?;
        let TaggedResponse { tag, response } = make_request(url)?;
        if tag != "redirect" {
            panic!("Not tagged as a Redirect Response: {:?}", response);
        }
        match response {
            Redirect(redirect) => {
                // eprintln!("Redirect: {:?}", redirect);
                assert_eq!("31 gemini://localhost/", redirect.status());
                assert_eq!("31", redirect.code());
                assert_eq!("gemini://localhost/", redirect.url());
            }
            _ => panic!("Unexpected Response: {:?}", response),
        }
        Ok(())
    }
    #[test]
    fn test_fun_server_temporary_failure() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let url = url::Url::parse("gemini://localhost/temporary%20failure")?;
        let TaggedResponse { tag, response } = make_request(url)?;
        if tag != "failure" {
            panic!("Not tagged as a Failure Response: {:?}", response);
        }
        match response {
            Failure(failure) => {
                // eprintln!("Failure: {:?}", failure);
                assert_eq!("40 Temporary failure", failure.status());
                assert_eq!("40", failure.code());
                assert_eq!("Temporary failure", failure.message());
            }
            _ => panic!("Unexpected Response: {:?}", response),
        }
        Ok(())
    }
    #[test]
    fn test_fun_server_server_unavailable() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let url = url::Url::parse("gemini://localhost/server%20unavailable")?;
        let TaggedResponse { tag, response } = make_request(url)?;
        if tag != "failure" {
            panic!("Not tagged as a Failure Response: {:?}", response);
        }
        match response {
            Failure(failure) => {
                // eprintln!("Failure: {:?}", failure);
                assert_eq!("41 Server unavailable", failure.status());
                assert_eq!("41", failure.code());
                assert_eq!("Server unavailable", failure.message());
            }
            _ => panic!("Unexpected Response: {:?}", response),
        }
        Ok(())
    }
    #[test]
    fn test_fun_server_cgi_error() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let url = url::Url::parse("gemini://localhost/cgi%20error")?;
        let TaggedResponse { tag, response } = make_request(url)?;
        if tag != "failure" {
            panic!("Not tagged as a Failure Response: {:?}", response);
        }
        match response {
            Failure(failure) => {
                // eprintln!("Failure: {:?}", failure);
                assert_eq!("42 CGI error", failure.status());
                assert_eq!("42", failure.code());
                assert_eq!("CGI error", failure.message());
            }
            _ => panic!("Unexpected Response: {:?}", response),
        }
        Ok(())
    }
    #[test]
    fn test_fun_server_proxy_error() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let url = url::Url::parse("gemini://localhost/proxy%20error")?;
        let TaggedResponse { tag, response } = make_request(url)?;
        if tag != "failure" {
            panic!("Not tagged as a Failure Response: {:?}", response);
        }
        match response {
            Failure(failure) => {
                // eprintln!("Failure: {:?}", failure);
                assert_eq!("43 Proxy error", failure.status());
                assert_eq!("43", failure.code());
                assert_eq!("Proxy error", failure.message());
            }
            _ => panic!("Unexpected Response: {:?}", response),
        }
        Ok(())
    }
    #[test]
    fn test_fun_server_slow_down() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let url = url::Url::parse("gemini://localhost/slow%20down")?;
        let TaggedResponse { tag, response } = make_request(url)?;
        if tag != "failure" {
            panic!("Not tagged as a Failure Response: {:?}", response);
        }
        match response {
            Failure(failure) => {
                // eprintln!("Failure: {:?}", failure);
                assert_eq!("44 Slow down", failure.status());
                assert_eq!("44", failure.code());
                assert_eq!("Slow down", failure.message());
            }
            _ => panic!("Unexpected Response: {:?}", response),
        }
        Ok(())
    }
    #[test]
    fn test_fun_server_permanent_failure() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let url = url::Url::parse("gemini://localhost/permanent%20failure")?;
        let TaggedResponse { tag, response } = make_request(url)?;
        if tag != "failure" {
            panic!("Not tagged as a Failure Response: {:?}", response);
        }
        match response {
            Failure(failure) => {
                // eprintln!("Failure: {:?}", failure);
                assert_eq!("50 Permanent failure", failure.status());
                assert_eq!("50", failure.code());
                assert_eq!("Permanent failure", failure.message());
            }
            _ => panic!("Unexpected Response: {:?}", response),
        }
        Ok(())
    }
    #[test]
    fn test_fun_server_not_found() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let url = url::Url::parse("gemini://localhost/bogus")?;
        let TaggedResponse { tag, response } = make_request(url)?;
        if tag != "failure" {
            panic!("Not tagged as a Failure Response: {:?}", response);
        }
        match response {
            Failure(failure) => {
                // eprintln!("Failure: {:?}", failure);
                assert_eq!("51 Not found", failure.status());
                assert_eq!("51", failure.code());
                assert_eq!("Not found", failure.message());
            }
            _ => panic!("Unexpected Response: {:?}", response),
        }
        Ok(())
    }
    #[test]
    fn test_fun_server_gone() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let url = url::Url::parse("gemini://localhost/gone")?;
        let TaggedResponse { tag, response } = make_request(url)?;
        if tag != "failure" {
            panic!("Not tagged as a Failure Response: {:?}", response);
        }
        match response {
            Failure(failure) => {
                // eprintln!("Failure: {:?}", failure);
                assert_eq!("52 Gone", failure.status());
                assert_eq!("52", failure.code());
                assert_eq!("Gone", failure.message());
            }
            _ => panic!("Unexpected Response: {:?}", response),
        }
        Ok(())
    }
    #[test]
    fn test_fun_server_proxy_request_refused() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let url = url::Url::parse("gemini://localhost/proxy%20request%20refused")?;
        let TaggedResponse { tag, response } = make_request(url)?;
        if tag != "failure" {
            panic!("Not tagged as a Failure Response: {:?}", response);
        }
        match response {
            Failure(failure) => {
                // eprintln!("Failure: {:?}", failure);
                assert_eq!("53 Proxy request refused", failure.status());
                assert_eq!("53", failure.code());
                assert_eq!("Proxy request refused", failure.message());
            }
            _ => panic!("Unexpected Response: {:?}", response),
        }
        Ok(())
    }
    #[test]
    fn test_fun_server_bad_request() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let bad_request = String::from_utf8_lossy(&[0x21_u8; 1024]).to_string();
        let url = url::Url::parse(format!("gemini://localhost/{}", bad_request).as_str())?;
        let TaggedResponse { tag, response } = make_request(url)?;
        if tag != "failure" {
            panic!("Not tagged as a Failure Response: {:?}", response);
        }
        match response {
            Failure(failure) => {
                // eprintln!("Failure: {:?}", failure);
                assert_eq!("59 Request is too large", failure.status());
                assert_eq!("59", failure.code());
                assert_eq!("Request is too large", failure.message());
            }
            _ => panic!("Unexpected Response: {:?}", response),
        }
        Ok(())
    }
}
*/
