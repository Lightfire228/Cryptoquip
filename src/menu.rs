use crate::website::image_contexts::ImageContext;


pub fn choose_image(images: Vec<ImageContext>) -> ImageContext {

    let menu_options: Vec<MenuOption> = images
        .into_iter()
        .map(|x| MenuOption::new(x))
        .collect()
    ;

    display_menu(&menu_options);

    todo!()
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

fn display_menu(menu_options: &Vec<MenuOption>) {

    let cols = calc_cols(&menu_options);

    for x in menu_options.iter() {
        println!("{}", x.get_formatted(&cols));
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

fn calc_cols(menu_options: &Vec<MenuOption>) -> ColSizes {
    let mut cols = ColSizes::new();
    for x in menu_options.iter() {
        cols.ord_col  = max(cols.ord_col,  x.ord_str() .len());
        cols.day_col  = max(cols.day_col,  x.day_str() .len());
        cols.date_col = max(cols.date_col, x.date_str().len());
    }

    cols
}
