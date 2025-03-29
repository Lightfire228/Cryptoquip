
mod website;
mod menu;

fn main() {
    println!("Hello, world!");
    let page = website::get_home_page();
    let images = website::get_image_contexts(&page);

    menu::choose_image(images);
}
