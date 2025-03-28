pub mod request;

use chrono::{DateTime, Local};

pub struct ImageContext {
    ordinal: usize,
    url:     String,
    date:    DateTime<Local>,
}

impl ImageContext {
    pub fn new() -> Self {
        Self {
            ordinal: 0,
            url:     String::from(""),
            date:    Local::now(),
        }
    }
}
