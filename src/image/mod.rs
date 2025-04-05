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
        self.find_black_line_row_segments();
    }

    pub fn find_black_line_row_segments(&self) -> Vec<Segment> {
        
        let mut black_row_start: Option<usize> = None;
        let mut segments = vec![];

        for y in 0..self.height {

            let start = self.get_ind(0, y);
            let end   = start + y;

            let is_black = is_row_black(&self.data[start..end]);

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

    fn iter_row<'a>(&'a self, y: usize) -> RowIter<'a> {
        RowIter {
            data:   &self,
            x:      0,
            row:    y,
        }
    }

    fn iter_col<'a>(&'a self, x: usize, row_start: usize, row_end: usize) -> ColIter<'a> {
        ColIter {
            data:   &self,
            y:      row_start,
            col:    x,
            row_end,
        }
    }

    fn get_ind(&self, x: usize, y: usize) -> usize {
        (y * self.height) + x
    }

    fn get(&self, x: usize, y: usize) -> &Pixel {
        &self.data[self.get_ind(x, y)]
    }
}


struct RowIter<'a> {
    data:  &'a RawImage,
    x:     usize,
    row:   usize,
}


impl<'a> Iterator for RowIter<'a> {
    type Item = &'a Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        
        if self.x >= self.data.width {
            return None;
        }
        
        let data  = Some(self.data.get(self.x, self.row));
        self.x   += 1;

        data
    }
}

struct ColIter<'a> {
    data:      &'a RawImage,
    y:         usize,
    col:       usize,
    row_end:   usize,
}


impl<'a> Iterator for ColIter<'a> {
    type Item = &'a Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        
        if self.y >= self.row_end {
            return None;
        }
        
        let data  = Some(self.data.get(self.col, self.y));
        self.y   += 1;

        data
    }
}

struct WalkRowIter<'a> {
    data:  &'a RawImage,
    y:     usize
}


impl<'a> Iterator for WalkRowIter<'a> {
    type Item = RowIter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        
        if self.y >= self.data.height {
            return None;
        }
        
        let data  = Some(self.data.iter_row(self.y));
        self.y   += 1;

        data
    }
}



fn is_black(pix: &Pixel) -> bool {
    *pix < 0x80
}

pub fn find_black_segments<T>(iter: T) -> Vec<Segment> 
    where T: Iterator
{
        
    let mut black_row_start: Option<usize> = None;
    let mut segments = vec![];

    for y in 0..self.height {

        let start = self.get_ind(0, y);
        let end   = start + y;

        let is_black = is_row_black(&self.data[start..end]);

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


