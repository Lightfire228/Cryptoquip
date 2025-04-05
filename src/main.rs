use std::{fs::File, io::BufWriter, path::Path};

use image::RawImage;
use png::Encoder;
use website::ImageContext;


mod website;
mod menu;
mod macros;
mod image;

fn main() {
    use menu::SelectionType::*;

    let page   = website::get_home_page();
    let images = website::get_image_contexts(&page);

    let chosen = menu::choose_image(images);

    match chosen {
        Image(ctx) => handle_selection(ctx),
        Quit       => return,
    }
}


fn handle_selection(ctx: ImageContext) {
    let mut raw_image = website::download_pdf_binary(ctx);

    dbg_write_to_png(&raw_image);

    raw_image.rectangulate();
}

fn dbg_write_to_png(raw_image: &RawImage) {

    let path = Path::new("./out/test.png");
    let file = File::create(path).unwrap();

    let width  = raw_image.width  as u32;
    let height = raw_image.height as u32;

    let ref mut w       = BufWriter::new(file);
    let     mut encoder = Encoder  ::new(w, width, height);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&raw_image.data).unwrap();
}