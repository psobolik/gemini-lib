use gemini_lib::response::{GeminiResponse, Response, Success};
use gemini_lib::parse_gemini_document::parse_gemini_document;

fn main() {
    let url = url::Url::parse("gemini://geminiprotocol.net/").unwrap();
    do_make_request(url.clone());
    do_make_gemini_request(url.clone());
}

fn do_make_gemini_request(url: url::Url) {
    println!("\x1b[93m** do_make_gemini_request **\x1b[0m");
    match gemini_lib::make_gemini_request(url) {
        Ok(response) => match response {
            Response::Prompt(prompt) => println!("{:?}", prompt),
            Response::Success(success) => print_success(success),
            Response::Redirect(redirect) => println!("{:?}", redirect),
            Response::Failure(failure) => println!("{:?}", failure),
            Response::Certificate(certificate) => println!("{:?}", certificate),
        }
        Err(error) => println!("\x1b[91mError: {}\x1b[0m", error),
    }
    println!();
}
fn do_make_request(url: url::Url) {
    println!("\x1b[93m** do_make_request **\x1b[0m");
    match gemini_lib::make_request(url) {
        Ok(GeminiResponse {
               prompt,
               success,
               redirect,
               failure,
               certificate,
           }) => {
            if let Some(prompt) = prompt { println!("{:?}", prompt) }
            if let Some(success) = success { print_success(success) }
            if let Some(redirect) = redirect { println!("{:?}", redirect) }
            if let Some(failure) = failure { println!("{:?}", failure) }
            if let Some(certificate) = certificate { println!("{:?}", certificate) }
        }
        Err(error) => println!("\x1b[91mError: {}\x1b[0m", error),
    }
    println!();
}

fn print_success(success: Success) {
    if success.is_gemini() {
        parse_gemini_document(success,
                              |preformatted_text| {
                                  println!("Preformatted text: {:?}", preformatted_text);
                              },
                              |link| {
                                  println!("Link: {:?}", link)
                              },
                              |(level, heading)| {
                                  println!("Level: {}; heading: {}", level, heading);
                              },
                              |list_item| {
                                  println!("List Item: {}", list_item);
                              },
                              |quoted_text| {
                                  println!("Quoted text: {}", quoted_text);
                              },
                              |plain_text| {
                                  println!("Plain text: {}", plain_text);
                              },
        )
    } else if success.is_text_like() {
        success.lines().into_iter().for_each(|line| println!("{}", line));
    } else {
        println!("\x1b[91mIgnoring mime-type '{}'\x1b[0m", success.mime_type());
    }
}