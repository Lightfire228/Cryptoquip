
use image::RawImage;
use website::ImageContext;


mod website;
mod menu;
mod macros;
mod image;
mod display;

#[cfg(feature = "cache")]
mod cache;

fn main() {

    if has_cache() {
        from_cache();
    }
    else {
        menu();
    }
}

fn has_cache() -> bool {
    #[cfg(feature = "cache")] {
        if cache::check_cache() {
            return true;
        }
    }

    false
}

fn menu() {
    use menu::SelectionType::*;

    let page   = website::get_home_page();
    let images = website::get_image_contexts(&page);

    let chosen = menu::choose_image(images);

    match chosen {
        Image(ctx) => handle_selection(&ctx),
        Quit       => return,
    }
}

#[allow(dead_code)]
fn from_cache() {
    #[cfg(feature = "cache")] {
        let data = cache::read_cache();
    
        let raw_image = website::from_cache(data);
    
        handle_image(raw_image, &ImageContext::from_cache());
    }
}


fn handle_selection(ctx: &ImageContext) {
    let raw_image = website::download_pdf_binary(ctx);

    handle_image(raw_image, ctx);

}

fn handle_image(raw_image: RawImage, ctx: &ImageContext) {
    let new_image = image::edit_image(raw_image, ctx);

    dbg_write_to_png(&new_image);

    display::display(&new_image);
}

fn dbg_write_to_png(_raw_image: &RawImage) {

    #[cfg(feature = "cache")] {

        use std::{fs::File, io::BufWriter, path::Path};
        use png::Encoder;

        let path = Path::new("./out/test.png");
        let file = File::create(path).unwrap();

        let width  = _raw_image.width  as u32;
        let height = _raw_image.height as u32;

        let ref mut w       = BufWriter::new(file);
        let     mut encoder = Encoder  ::new(w, width, height);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&_raw_image.data).unwrap();
    }

}