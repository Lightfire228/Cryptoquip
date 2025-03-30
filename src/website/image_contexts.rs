

use chrono::{DateTime, Datelike, FixedOffset, Weekday};
use scraper::{selectable::Selectable, ElementRef, Html, Selector};

type Elems<'a>      = Vec<ElementRef<'a>>;
type ParseResult<T> = Result<T, ParseErrorType>;


pub fn get_image_contexts(page: &str) -> Vec<ImageContext> {

    match (|| {
        let document = Html::parse_document(page);
    
        let cards    = extract_image_cards(&document)?;
        let contexts = to_image_contexts(&cards)?;
    
        Ok(contexts)
    })() {
        Ok(x)  => x,
        Err(e) => display_error(e),
    }
    
}


#[derive(Debug, Clone)]
pub struct ImageContext {
    pub ordinal: usize,
    pub url:     String,
    pub date:    DateTime<FixedOffset>,
    pub uuid:    String,
}

impl ImageContext {

    fn new(i: usize, card: &ElementRef) -> ParseResult<Self> {

        let url  = _extract_crypto_url(card)?;
        let uuid = _parse_uuid        (&url)?;
        let date = _extract_date      (card)?;

        Ok(Self {
            ordinal: i,
            url,
            date,
            uuid,
        })
    }

    pub fn is_sunday(&self) -> bool {
        self.date.weekday() == Weekday::Sun
    }

    pub fn day_str(&self) -> String {
        self.date.format("%A").to_string()
    }

    pub fn date_str(&self) -> String {
        self.date.format("%x").to_string()
    }

    pub fn formatted_date(&self) -> String {
        let day  = self.day_str();
        let date = self.date.format("%x").to_string();
        
        format!("{} - {}", day, date)
    }
}

enum ParseErrorType {
    ContentNotFound,
    CardBodyNotFound,
    AnchorTagNotFound,
    UrlNotFound,
    DateNotFound,
    DateTimeAttributeNotFound,
    DateTimeParseErr,
    UuidParseErr
}

fn display_error(err: ParseErrorType) -> ! {
    use ParseErrorType::*;

    match err {
        ContentNotFound           => panic!("Unable to find main content body"),
        CardBodyNotFound          => panic!("Cannot find card body tag within the image card"),
        AnchorTagNotFound         => panic!("Cannot find anchor tag from card body"),
        UrlNotFound               => panic!("Cannot find url from anchor tag"),
        DateNotFound              => panic!("Cannot find date within the image card"),
        DateTimeAttributeNotFound => panic!("Cannot find 'datetime' attribute from time tag"),
        DateTimeParseErr          => panic!("Cannot parse datetime"),
        UuidParseErr              => panic!("Unable to parse image UUID from url"),
    }
}

fn extract_image_cards<'a>(document: &'a Html) -> ParseResult<Elems<'a>> {

    let content_selector   = _get_selector("#main-page-container");
    let card_grid_selector = _get_selector("div .card-grid");
    let card_selector      = _get_selector("div .card-container");

    let content   = document.select(&content_selector).next()
        .ok_or(ParseErrorType::ContentNotFound)?
    ;
    let card_grid = content .select(&card_grid_selector).collect::<Elems>();

    let cards = card_grid
        .iter()
        .flat_map(|c| 
            c.select(&card_selector).collect::<Vec<ElementRef>>()
        )
        .collect::<Vec<ElementRef>>()
    ;

    Ok(cards)
}

fn _get_selector(str: &str) -> Selector {
    Selector::parse(str).unwrap()
}

fn to_image_contexts(cards: &Elems) -> ParseResult<Vec<ImageContext>> {
    cards
        .iter()
        .enumerate()
        .map(|(i, card)| ImageContext::new(i, card))
        .collect()
}

fn _extract_crypto_url(card: &ElementRef) -> ParseResult<String> {
    use ParseErrorType::*;

    let body_selector = _get_selector(".card-body");
    let a_selector    = _get_selector("a");

    let body = _select_first(&card, &body_selector).ok_or(CardBodyNotFound) ?;
    let a    = _select_first(&body, &a_selector)   .ok_or(AnchorTagNotFound)?;

    let href = a.value().attr("href").ok_or(UrlNotFound)?;

    Ok(String::from(href))
}

fn _extract_date(card: &ElementRef) -> ParseResult<DateTime<FixedOffset>> {
    use ParseErrorType::*;

    let time_selector = _get_selector("time");

    let time = _select_first(&card, &time_selector).ok_or(DateNotFound)             ?;
    let iso  = time.value().attr("datetime")       .ok_or(DateTimeAttributeNotFound)?;

    let parsed = DateTime::parse_from_rfc3339(iso).map_err(|_| DateTimeParseErr)?;

    Ok(parsed)
}

fn _select_first<'a>(el: &'a ElementRef, selector: &Selector) -> Option<ElementRef<'a>> {
    Some(el.select(selector).next()?.to_owned())
}

fn _select_all<'a>(el: &'a ElementRef, selector: &Selector) -> Vec<ElementRef<'a>> {
    el.select(selector).collect()
}

fn _get_attr<'a>(el: &'a ElementRef, attr: &str) -> Option<String> {
    let val = el.value().attr(attr)?;

    Some(String::from(val))
}

fn _parse_uuid(url: &str) -> ParseResult<String> {
    use ParseErrorType::*;
        
    let uuid = url.split('/').last().ok_or(UuidParseErr)?;

    Ok(uuid
        .replace("file_", "")
        .replace(".html", "")
    )
}