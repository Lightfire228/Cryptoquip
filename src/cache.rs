use chrono::Local;
use std::{fs::{read, write}, path::Path};

use crate::website::ImageContext;

static CACHE: &str = "./out/cache.pdf";

pub fn check_cache() -> bool {
    let path = Path::new(CACHE);

    path.exists()
}

pub fn write_cache(data: &[u8]) {
    write(CACHE, data).unwrap()
}

pub fn read_cache() -> Vec<u8> {
    read(CACHE).unwrap()

}


impl ImageContext {
    pub fn from_cache() -> Self {
        ImageContext { ordinal: 0, url: "".to_owned(), date: Local::now().into() }
    }

}

