use scraper::Html;
use std::env;

use super::{
    get_selector,
    select_first,
    get_attr,
    ImageContext,
    request::{get_image_page, get_pdf}, 
};

type ParseResult<T> = Result<T, ParseErrorType>;


pub fn download_pdf_binary(ctx: ImageContext) {

    match (|| {
        let page      = get_image_page(&ctx);
        let pdf_url   = extract_pdf_url(&page)?;
        let pdf_bytes = get_pdf(&pdf_url);

        Ok(())

    })() {
        Ok(x)    => x,
        Err(err) => display_error(err)
    }
}

enum ParseErrorType {
    ContentNotFound,
    AnchorNotFound,
    HrefAttrNotFound,
}


fn display_error(err: ParseErrorType) -> ! {
    use ParseErrorType::*;

    eprintln!("Error while trying to download PDF");

    match err {
        ContentNotFound  => panic!("Cannot find main content body"),
        AnchorNotFound   => panic!("Cannot find anchor tag"),
        HrefAttrNotFound => panic!("Cannot find url from anchor tag"),
    }
}

fn extract_pdf_url(page: &str) -> ParseResult<String> {
    use ParseErrorType::*;

    let content_selector = get_selector("#asset-content");
    let anchor_selector  = get_selector("a");

    let document = Html::parse_document(page);
    let document = document.root_element();

    let content  = select_first(&document, &content_selector).ok_or(ContentNotFound) ?;
    let anchor   = select_first(&content,  &anchor_selector) .ok_or(AnchorNotFound)  ?;
    let href     = get_attr    (&anchor,   "href")           .ok_or(HrefAttrNotFound)?;

    Ok(href.to_owned())
}

fn save_pdf(bytes: Vec<u8>) {
    let dir = env::temp_dir();

    // let 
    todo!()
}