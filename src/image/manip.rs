use std::convert::Infallible;

use crate::website::ImageContext;

use super::{RawImage, Segment};

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

    pub fn rectangulate(&mut self) {
        let segments = self.find_black_line_row_segments();

        println!("{}", segments.len());

        for s in segments.iter() {

            dbg!(s);
            for y in s.start..s.end {
                for x in 0..self.width {
                    let pix = self.get_mut(x, y);
                    *pix = 0;
                }
            }
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
                black_row_start = None;
            }
        }

        match black_row_start {
            None    => (),
            Some(x) => segments.push(Segment::new(x, self.height)),
        }

        segments

    }
}