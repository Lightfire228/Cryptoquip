use std::{fs::{read, write}, path::Path};

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
