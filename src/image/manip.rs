use std::convert::Infallible;

use crate::{image::Point, website::ImageContext};

use super::{RawImage, Rect, Segment};

pub fn edit_image(img: &mut RawImage, _ctx: &ImageContext) {
    match (|| {

        img.rectangulate();

        Ok::<_, Infallible>(())
    })() {
        Ok(()) => {}
    }
}



// Take the image processing code straight from python

impl RawImage {

    pub fn rectangulate(&self) -> Vec<Rect> {

        let mut rects = Vec::new();

        for row in self.find_black_line_row_segments() {
            for col in self.find_black_line_col_segments(&row) {

                rects.push(Rect {
                    top_left:     Point { x: col.start, y: row.start },
                    bottom_right: Point { x: col.end,   y: row.end   },
                });
            }
        }

        rects
    }

    pub fn find_black_line_row_segments(&self) -> Vec<Segment> {
        self.compactify(self.height, |x| self.is_row_black(x))
    }

    pub fn find_black_line_col_segments(&self, col_height: &Segment) -> Vec<Segment> {
        self.compactify(self.width, |x| self.is_col_black(x, col_height))
    }

    pub fn compactify<F>(&self, max: usize, is_span_black: F) -> Vec<Segment> 
        where F: Fn(usize) -> bool
    {
        
        let mut segment_start: Option<usize> = None;
        let mut segments = vec![];

        for i in 0..max {

            let is_black = is_span_black(i);

            let start_black_segment =  is_black && segment_start.is_none();
            let start_white_segment = !is_black && segment_start.is_some();

            if start_black_segment {
                segment_start = Some(i);
            }
            else if start_white_segment {
                segments.push(Segment::new(segment_start.unwrap(), i));
                segment_start = None;
            }
        }

        match segment_start {
            None    => (),
            Some(i) => segments.push(Segment::new(i, max)),
        }

        segments

    }

}


