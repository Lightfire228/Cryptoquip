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
    website::download_pdf_binary(ctx);
}