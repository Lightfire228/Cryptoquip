
mod website;

fn main() {
    println!("Hello, world!");
    let page = website::get_home_page();
    website::get_image_contexts(&page);
}
