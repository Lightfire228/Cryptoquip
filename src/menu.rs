use std::{io::{self, Write}, num::IntErrorKind};

use crate::{multi_line, website::image_contexts::ImageContext};

type BiMenu<'a>          = (Vec<&'a MenuOption>, Vec<&'a MenuOption>);
type MenuResult<T>       = Result<T,     MenuErrorType>; 

pub fn choose_image(images: Vec<ImageContext>) -> Option<ImageContext> {
    use MenuErrorType::*;

    match (|| {

        let menu_options: Vec<MenuOption> = images
            .into_iter()
            .map(|x| MenuOption::new(x))
            .collect()
        ;
    
        if menu_options.len() == 0 {
            Err(NoImagesFound)?
        }
    
        display_menu(
            &bifurcate(menu_options.iter().collect())
        );

        let index = get_user_selection(menu_options.len()).ok_or(Quit)?;

        Ok(menu_options[index].context.clone())

    })() {
        Err(Quit) => None,
        Ok (x)    => Some(x),
        Err(err)  => display_error(err),
    }

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

enum MenuErrorType {
    StdInFailed,
    NoImagesFound,
    StdOutFlushFailed,
    Quit,
}

enum UserSelectionType {
    NumberFormatErr,
    NumberOutOfRange(usize),
    IOErr(MenuErrorType),
    Quit,
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

fn display_error(err: MenuErrorType) -> ! {
    use MenuErrorType::*;

    match err {
        StdInFailed         => panic!("Unable to read from stdin"),
        StdOutFlushFailed   => panic!("Unable to flush stdout"),
        NoImagesFound       => panic!("No images found"),
        Quit                => panic!("User Quit (Unreachable)"),
    }
}

// TODO: Should prolly extract this into it's own module
fn get_user_selection(menu_len: usize) -> Option<usize> {
    use UserSelectionType::*;

    fn get_usr_in() -> Result<String, UserSelectionType> {
        Ok(get_stdin().map_err(|e| IOErr(e))?)
    }

    fn check_quit_input(usr_in: &str) -> Result<(), UserSelectionType> {
        if usr_in.to_lowercase() == "q" {
            return Err(Quit);
        }

        Ok(())
    }

    fn convert_usize (usr_in: &str, menu_len: usize) -> Result<usize, UserSelectionType> {
        match usr_in.parse::<usize>() {
            Err(err) => match err.kind() {
                // If no input, default to 0
                IntErrorKind::Empty       => Ok(0),

                IntErrorKind::NegOverflow => Err(NumberOutOfRange(menu_len)),
                IntErrorKind::PosOverflow => Err(NumberOutOfRange(menu_len)),
                _                         => Err(NumberFormatErr),
            },
            Ok(i) => Ok(i),
        }
    }

    fn check_range (index: usize, menu_len: usize) -> Result<usize, UserSelectionType> {
        if index >= menu_len {
            return Err(NumberOutOfRange(menu_len));
        }

        Ok(index)
    }

    loop {

        match (|| {

            let usr_in = get_usr_in()?;
            let usr_in = usr_in.trim();

            check_quit_input(&usr_in)?;

            let index = convert_usize(&usr_in, menu_len)?;

            check_range(index, menu_len)?;

            Ok(index)
        })() {
            Ok (x)    => return Some(x),
            Err(Quit) => return None,
            Err(err)  => display_menu_error(err),
        }
    }

}

fn display_menu_error(err: UserSelectionType) {
    use UserSelectionType::*;

    match err {
        NumberFormatErr             => println!("Unknown input (must be a number or Q)"),
        NumberOutOfRange(menu_len)  => println!("Number out of bounds, must be 0 to {}", menu_len -1),
        IOErr(err)                  => display_error(err),
        Quit                        => ()
    }
}

fn get_stdin() -> MenuResult<String> {
    use MenuErrorType::*;

    print!("> ");
    io::stdout().flush().map_err(|_| StdOutFlushFailed)?;

    let mut buffer = String::new();
    let     stdin  = io::stdin();

    stdin.read_line(&mut buffer).map_err(|_| StdInFailed)?;

    Ok(buffer)
}

fn display_menu(bi_menu: &BiMenu) {

    println!("{}", multi_line!(
        "Which Crytoquip to download?",
        " - press Enter to download the most recent",
        " - press q to Quit",
        "",
    ));

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
        if i % 2 == 0 { bi_menu.0.push(x) }
        else          { bi_menu.1.push(x) }
    }

    bi_menu
}