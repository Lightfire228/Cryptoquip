mod rectangulate;
mod edit;
mod parsed_image;
mod raw_image;


pub use edit::edit_image;
pub use raw_image::RawImage;

type Pixel = u8;


pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub top_left:     Point,
    pub bottom_right: Point,
}

#[derive(Debug)]
pub struct Segment {
    pub start: usize,
    pub end:   usize,
}

impl Segment {
    pub fn new(start: usize, end: usize) -> Self {
        if start > end {
            panic!("Bad segment: start ({}) must be <= end ({})", start, end);
        }

        Segment { start, end }
    }
}
