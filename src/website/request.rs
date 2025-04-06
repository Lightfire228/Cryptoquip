use std::io::Read;

use reqwest::{blocking::{RequestBuilder, Response}, header::USER_AGENT, StatusCode};

use super::image_contexts::ImageContext;

static URL: &str = "https://www.cecildaily.com/diversions/cryptoquip/";
static UA:  &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:79.0) Gecko/20100101 Firefox/79.0";


pub fn get_home_page() -> String {
    get(URL)
}

pub fn get_image_page(image: &ImageContext) -> String {

    let url = &image.url;
    let url = url.strip_prefix("/").unwrap_or(&url);
    let url = format!("{}{}", URL, url);

    get(&url)
}

pub fn get_pdf(url: &str) -> Vec<u8> {
    get_as_bytes(url)
}


#[allow(non_camel_case_types)]
enum MakeRequestError {
    RequestFailed,
    HTTP_Status(StatusCode),
    ReadBodyStringFailed,
    ReadBodyBinaryFailed,

}

fn make_request(url: &str) -> Result<Response, MakeRequestError> {
    use MakeRequestError::*;

    let res = build_request(url).send()
        .map_err(|_| RequestFailed )?
    ;

    if res.status().is_client_error() {
        return Err(HTTP_Status(res.status()));
    }

    Ok(res)
}

fn make_request_string(url: &str) -> Result<String, MakeRequestError> {
    use MakeRequestError::*;

    let mut res = make_request(url)?;

    let mut body = String::new();
    res.read_to_string(&mut body)
        .map_err(|_| ReadBodyStringFailed)?
    ;

    Ok(body)
}

fn make_request_bytes(url: &str) -> Result<Vec<u8>, MakeRequestError> {
    use MakeRequestError::*;

    let mut res = make_request(url)?;

    let mut body = Vec::new();
    res.read_to_end(&mut body)
        .map_err(|_| ReadBodyBinaryFailed)?
    ;

    Ok(body)
}

fn get(url: &str) -> String {
    get_as(url, &make_request_string)
}

fn get_as_bytes(url: &str) -> Vec<u8> {
    get_as(url, &make_request_bytes)
}

fn get_as<T>(url: &str, make_request: &dyn Fn(&str) -> Result<T, MakeRequestError>) -> T {
    use MakeRequestError::*;

    let res = make_request(url);

    match res {
        Err(t) => { 
            eprintln!("Url: '{}'", url);
            match t {
                RequestFailed        => panic!("Unable to make request"),
                HTTP_Status(status)  => panic!("Returned status '{}'", status.as_u16()),
                ReadBodyStringFailed => panic!("Unable to parse request body as UTF-8"),
                ReadBodyBinaryFailed => panic!("Unable to parse request body as bytes"),
            }
        }

        Ok(r) => r,
    }
}

fn build_request(url: &str) -> RequestBuilder {
    let client = reqwest::blocking::Client::new();
    
    // need to set the UA, or some requests will fail with a 429
    client.get(url).header(USER_AGENT, UA)
}