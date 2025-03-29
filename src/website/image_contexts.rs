

use chrono::{DateTime, Datelike, FixedOffset, Weekday};
use scraper::{selectable::Selectable, ElementRef, Html, Selector};

type Elems<'a> = Vec<ElementRef<'a>>;


pub fn get_image_contexts(page: &str) -> Vec<ImageContext> {

    let document = Html::parse_document(page);

    let cards    = extract_image_cards(&document);
    let contexts = to_image_contexts(&cards);

    contexts
}


#[derive(Debug, Clone)]
pub struct ImageContext {
    pub ordinal: usize,
    pub url:     String,
    pub date:    DateTime<FixedOffset>,
}

impl ImageContext {
    pub fn is_sunday(&self) -> bool {
        self.date.weekday() == Weekday::Sun
    }

    pub fn day_str(&self) -> String {
        self.date.format("%A").to_string()
    }

    pub fn date_str(&self) -> String {
        self.date.format("%x").to_string()
    }

    pub fn uuid(&self) -> String {
        
        let uuid = self.url.split('/').last().expect("Unable to parse image UUID from url");

        uuid
            .replace("file_", "")
            .replace(".html", "")
    }

    pub fn formatted_date(&self) -> String {
        let day  = self.day_str();
        let date = self.date.format("%x").to_string();
        
        format!("{} - {}", day, date)
    }
}


fn extract_image_cards<'a>(document: &'a Html) -> Elems<'a> {

    let content_selector   = Selector::parse("#main-page-container").unwrap();
    let card_grid_selector = Selector::parse("div .card-grid")      .unwrap();
    let card_selector      = Selector::parse("div .card-container") .unwrap();

    let content   = document.select(&content_selector).next().expect("Unable to find main content body");
    let card_grid = content .select(&card_grid_selector).collect::<Elems>();

    let cards = card_grid
        .iter()
        .flat_map(|c| 
            c.select(&card_selector).collect::<Vec<ElementRef>>()
        )
        .collect::<Vec<ElementRef>>()
    ;

    cards
}

fn to_image_contexts(cards: &Elems) -> Vec<ImageContext> {
    cards
        .iter()
        .enumerate()
        .map(|c| {
            ImageContext {
                ordinal: c.0,
                url:     _extract_crypto_url(c.1),
                date:    _extract_date      (c.1),
            }
        })
        .collect()
}

fn _extract_crypto_url(card: &ElementRef) -> String {

    let body_selector = Selector::parse(".card-body").unwrap();
    let a_selector    = Selector::parse("a")         .unwrap();

    let body = _select_first(&card, &body_selector, "Cannot find card body tag within the image card");
    let a    = _select_first(&body, &a_selector,    "Cannot find anchor tag from card body");

    let href = a.value().attr("href").expect("Cannot find url from anchor tag");

    String::from(href)
}

fn _extract_date(card: &ElementRef) -> DateTime<FixedOffset> {

    let time_selector = Selector::parse("time").unwrap();

    let time = _select_first(&card, &time_selector, "Cannot find date within the image card");

    let iso  = time.value().attr("datetime").expect("Cannot find 'datetime' attribute from time tag");

    let parsed = DateTime::parse_from_rfc3339(iso).expect("Cannot parse datetime");

    parsed
}

fn _select_first<'a>(el: &'a ElementRef, selector: &Selector, msg: &str) -> ElementRef<'a> {
    el.select(selector).next().expect(msg).to_owned()
}

fn _select_all<'a>(el: &'a ElementRef, selector: &Selector) -> Vec<ElementRef<'a>> {
    el.select(selector).collect()
}

fn _get_attr<'a>(el: &'a ElementRef, attr: &str, msg: &str) -> String {
    let val = el.value().attr(attr).expect(msg);

    String::from(val)
}