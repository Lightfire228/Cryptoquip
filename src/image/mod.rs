use std::slice::Iter;

use pdf::object::ImageDict;

type Pixel = u8;

#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize
}

#[derive(Debug)]
pub struct Rect {
    pub start: Point,
    pub end:   Point,
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
    
    // Take the image processing code straight from python

    pub fn rectangulate(&mut self) {
        let segments = self.find_black_line_row_segments();

        let mut black = true;

        for s in segments {
            
            for y in s.start..s.end {
                for x in 0..self.width {
                    let pix = self.get_mut(x, y);

                    *pix = if black { 0 } else { 255 };
                }
            }

            black ^= true;
        }
    }

    // skip 1, take 1
    // 127 to 174

    pub fn find_black_line_row_segments(&self) -> Vec<Segment> {
        
        let mut black_row_start: Option<usize> = None;
        let mut segments = vec![];

        for y in 0..self.height {

            let is_black = self.is_row_black(y);

            let start_black_segment =  is_black && black_row_start.is_none();
            let start_white_segment = !is_black && black_row_start.is_some();

            if start_black_segment {
                black_row_start = Some(y);
            }
            else if start_white_segment {
                segments.push(Segment::new(black_row_start.unwrap(), y));
            }
        }

        match black_row_start {
            None    => (),
            Some(x) => segments.push(Segment::new(x, self.height)),
        }

        segments

    }

    fn is_row_black(&self, row: usize) -> bool {

        for x in 0..self.width {

            if is_black(self.get(x, row)) {
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

