use std::convert::Infallible;

use crate::{image::Point, website::ImageContext};

use super::{is_black, Pixel, RawImage, Rect, Segment};

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

    pub fn rectangulate(&mut self) -> Vec<Rect> {

        let mut rects = Vec::new();

        let row_segments = self.find_black_line_row_segments();

        for row in row_segments {

            let col_segments = self.find_black_line_col_segments(&row);

            for col in col_segments {

                rects.push(Rect {
                    top_left:     Point { x: col.start, y: row.start },
                    bottom_right: Point { x: col.end,   y: row.end   },
                });
            }
        }

        for r in rects.iter() {
            dbg!(&r);
            for y in r.top_left.y..r.bottom_right.y {
                for x in r.top_left.x..r.bottom_right.x {


                    let pix = self.get_mut(x, y);

                    (*pix) = 0;
                }
            }

        }

        rects
    }

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
                black_row_start = None;
            }
        }

        match black_row_start {
            None    => (),
            Some(x) => segments.push(Segment::new(x, self.height)),
        }

        segments

    }

    pub fn find_black_line_col_segments(&self, col_height: &Segment) -> Vec<Segment> {
        
        let mut black_col_start: Option<usize> = None;
        let mut segments = vec![];

        for x in 0..self.width {

            let is_black = self.is_col_black(x, col_height);

            let start_black_segment =  is_black && black_col_start.is_none();
            let start_white_segment = !is_black && black_col_start.is_some();

            if start_black_segment {
                black_col_start = Some(x);
            }
            else if start_white_segment {
                segments.push(Segment::new(black_col_start.unwrap(), x));
                black_col_start = None;
            }
        }

        match black_col_start {
            None    => (),
            Some(x) => segments.push(Segment::new(x, self.width)),
        }

        segments

    }
}