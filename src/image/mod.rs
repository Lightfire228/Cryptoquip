mod rectangulate;
mod edit;
mod parsed_image;
mod raw_image;
mod scale;


pub use edit::edit_image;
pub use raw_image::RawImage;
pub use scale::scale;

type Pixel = u8;

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


impl Rect {
    pub fn get_height(&self) -> usize {
        self.bottom_right.y - self.top_left.y 
    }

    pub fn _get_width(&self) -> usize {
        self.bottom_right.x - self.top_left.x 
    }
}



pub struct RectIter<'a> {
    rect: &'a Rect,

    x: usize,
    y: usize,
}

impl<'a> Iterator for RectIter<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {

        

        if self.y >= self.rect.bottom_right.y {
            return None;
        }

        let (x, y) = (self.x, self.y);


        self.x += 1;

        if self.x >= self.rect.bottom_right.x {
            self.x  = self.rect.top_left.x;
            self.y += 1
        }

        Some(Point { x, y, })
    }
}

fn iter_rect<'a>(rect: &'a Rect) -> RectIter<'a> {
    RectIter { 
        rect,
        x: rect.top_left.x, 
        y: rect.top_left.y,
    }
}