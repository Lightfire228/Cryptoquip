
mod website;
mod menu;
mod macros;

fn main() {
    let page = website::get_home_page();
    let images = website::get_image_contexts(&page);

    menu::choose_image(images);
}
