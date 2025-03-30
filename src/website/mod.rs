pub mod request;
pub mod image_contexts;

pub use request::get_home_page;
pub use request::get_image_page;
pub use image_contexts::get_image_contexts;

use scraper::ElementRef;
use scraper::Selector;

fn select_first<'a>(el: &'a ElementRef, selector: &Selector) -> Option<ElementRef<'a>> {
    Some(el.select(selector).next()?.to_owned())
}

fn select_all<'a>(el: &'a ElementRef, selector: &Selector) -> Vec<ElementRef<'a>> {
    el.select(selector).collect()
}

fn get_attr<'a>(el: &'a ElementRef, attr: &str) -> Option<String> {
    let val = el.value().attr(attr)?;

    Some(String::from(val))
}

fn get_selector(str: &str) -> Selector {
    Selector::parse(str).unwrap()
}
