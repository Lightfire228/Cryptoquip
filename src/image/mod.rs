mod manip;

use pdf::object::ImageDict;

pub use manip::edit_image;

type Pixel = u8;

#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct RawImage {
    pub data:     Vec<Pixel>,
    pub width:    usize,
    pub height:   usize,
}

impl RawImage {
    pub fn from_pdf(data: Vec<Pixel>, img_dict: ImageDict) -> Self {
        Self {
            data,
            width:  img_dict.width  as usize,
            height: img_dict.height as usize,
        }
    }
   
    pub fn is_row_black(&self, row: usize) -> bool {

        for x in 0..self.width {

            if is_black(self.get(x, row)) {
                return true;
            }
        }
    
        false
    }

    pub fn is_col_black(&self, col: usize, col_height: &Segment) -> bool {

        for y in col_height.start..col_height.end {

            if is_black(self.get(col, y)) {
                return true;
            }
        }
    
        false
    }

    fn get_ind(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }

    fn get(&self, x: usize, y: usize) -> &Pixel {
        &self.data[self.get_ind(x, y)]
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut Pixel {
        let ind = self.get_ind(x, y);
        &mut self.data[ind]
    }
}




fn is_black(pix: &Pixel) -> bool {
    *pix < 0x80
}

