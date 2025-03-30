use std::io::Read;

use reqwest::StatusCode;

use super::image_contexts::ImageContext;

static URL: &str = "https://www.cecildaily.com/diversions/cryptoquip/";


pub fn get_home_page() -> String {
    get(URL)
}

pub fn get_image_page(image: ImageContext) {
    
}


#[allow(non_camel_case_types)]
enum MakeRequestError {
    RequestFailed,
    HTTP_Status(StatusCode),
    InvalidUTF8,

}

fn _make_request(url: &str) -> Result<String, MakeRequestError> {
    use MakeRequestError::*;

    let mut res = reqwest::blocking::get(url)
        .map_err(|_| RequestFailed )?
    ;

    if res.status().is_client_error() {
        return Err(HTTP_Status(res.status()));
    }

    let mut body = String::new();
    res.read_to_string(&mut body)
        .map_err(|_| InvalidUTF8 )?
    ;

    Ok(body)
}

fn get(url: &str) -> String {
    use MakeRequestError::*;
    let res = _make_request(url);

    match res {
        Err(t) => { 
            eprintln!("Url: '{}'", url);
            match t {
                RequestFailed       => panic!("Unable to make request"),
                HTTP_Status(status) => panic!("Returned status '{}'", status.as_u16()),
                InvalidUTF8         => panic!("Returned invalid UTF-8"),
            }
        }

        Ok(r) => r,
    }
}