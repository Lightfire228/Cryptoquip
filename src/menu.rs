use std::convert::Infallible;

use crate::website::image_contexts::ImageContext;

type BiMenu<'a> = (Vec<&'a MenuOption>, Vec<&'a MenuOption>);


pub fn choose_image(images: Vec<ImageContext>) -> ImageContext {

    let menu_options: Vec<MenuOption> = images
        .into_iter()
        .map(|x| MenuOption::new(x))
        .collect()
    ;

    assert!(menu_options.len() > 0, "No images found");

    display_menu(
        &bifurcate(menu_options.iter().collect())
    );

    menu_options[0].context.clone()
}


pub struct MenuOption {
    context: ImageContext,
}

pub struct ColSizes {
    pub ord_col:  usize,
    pub day_col:  usize,
    pub date_col: usize,
}

impl ColSizes { 
    pub fn new() -> Self {
        Self { 
            ord_col:  0,
            day_col:  0,
            date_col: 0,
        }
    }
}


impl MenuOption {
    pub fn new(context: ImageContext) -> Self {
        Self {
            context,
        }
    }

    pub fn ordinal(&self) -> usize {
        self.context.ordinal
    }

    pub fn ord_str(&self) -> String {
        format!("{}", self.ordinal())
    }

    pub fn day_str(&self) -> String {
        self.context.day_str()
    }

    pub fn date_str(&self) -> String {
        self.context.date_str()
    }

    pub fn get_formatted(&self, col_sizes: &ColSizes) -> String {
        let ord  = align(&self.ord_str(),  col_sizes.ord_col,  Align::Right);
        let day  = align(&self.day_str(),  col_sizes.day_col,  Align::Left);
        let date = align(&self.date_str(), col_sizes.date_col, Align::Left);

        format!("{} - {} - {}", ord, day, date)
    }
}

enum Align {
    Right,
    Left,
}

fn display_menu(bi_menu: &BiMenu) {

    let col_1 = &bi_menu.0;
    let col_2 = &bi_menu.1;

    let col_size_1 = calc_cols(&col_1);
    let col_size_2 = calc_cols(&col_2);


    for (first, second) in col_1.into_iter().zip(col_2) {
        let first_str  = first .get_formatted(&col_size_1);
        let second_str = second.get_formatted(&col_size_2);
    
        println!("  {} | {}", first_str, second_str);
    }

    if col_1.len() > col_2.len() {
        println!("  {}", col_1.last().unwrap().get_formatted(&col_size_1));
    }
}

fn align(text: &str, padding: usize, align: Align) -> String {

    let padding = padding - text.len();
    
    // right adjust adds padding to the left of the word
    let (left_padding, right_padding) = match align {
        Align::Left  => (0,       padding),
        Align::Right => (padding, 0),
    };

    let left_space  = " ".repeat(left_padding);
    let right_space = " ".repeat(right_padding);

    format!("{}{}{}", left_space, text, right_space)

}

fn max(a: usize, b: usize) -> usize {
    if a > b {a} else {b}
}

fn calc_cols(menu_options: &Vec<&MenuOption>) -> ColSizes {
    let mut cols = ColSizes::new();
    for x in menu_options.iter() {
        cols.ord_col  = max(cols.ord_col,  x.ord_str() .len());
        cols.day_col  = max(cols.day_col,  x.day_str() .len());
        cols.date_col = max(cols.date_col, x.date_str().len());
    }

    cols
}

fn bifurcate(menu_options: Vec<&MenuOption>) -> BiMenu {

    let mut bi_menu = (Vec::new(), Vec::new());

    for (i, x) in menu_options.into_iter().enumerate() {
        match i % 2 {
            0 => bi_menu.0.push(x),
            1 => bi_menu.1.push(x),
            _ => assert!(false),
        }
    }

    bi_menu
}