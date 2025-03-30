use website::image_contexts::ImageContext;


mod website;
mod menu;
mod macros;

fn main() {
    use menu::SelectionType::*;

    let page   = website::get_home_page();
    let images = website::get_image_contexts(&page);

    let chosen = menu::choose_image(images);

    match chosen {
        Image(ctx) => handle_selection(ctx),
        Quit       => ()
    }
}


fn handle_selection(image: ImageContext) {

}