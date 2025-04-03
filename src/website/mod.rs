mod request;
mod image_contexts;
mod pdf;

pub use request::get_home_page;
pub use image_contexts::{get_image_contexts, ImageContext};
pub use pdf::download_pdf_binary;

use scraper::ElementRef;
use scraper::Selector;

fn select_first<'a>(el: &'a ElementRef, selector: &Selector) -> Option<ElementRef<'a>> {
    Some(el.select(selector).next()?.to_owned())
}

fn get_attr<'a>(el: &'a ElementRef, attr: &str) -> Option<String> {
    let val = el.value().attr(attr)?;

    Some(val.to_owned())
}

fn get_selector(str: &str) -> Selector {
    Selector::parse(str).unwrap()
}
