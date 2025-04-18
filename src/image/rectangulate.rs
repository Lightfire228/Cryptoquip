
use super::{Point, RawImage, Rect, Segment};

impl RawImage {

    pub fn rectangulate(&self) -> Vec<Vec<Rect>> {

        let mut rects = Vec::new();
        for row in self.find_black_line_row_segments() {

            let mut row_rects = Vec::new();
            for col in self.find_black_line_col_segments(&row) {

                row_rects.push(Rect {
                    top_left:     Point { x: col.start, y: row.start },
                    bottom_right: Point { x: col.end,   y: row.end   },
                });
            }

            rects.push(row_rects);
        }

        rects
    }

    fn find_black_line_row_segments(&self) -> Vec<Segment> {
        self.segmentify(self.height, |x| self.is_row_black(x))
    }

    fn find_black_line_col_segments(&self, col_height: &Segment) -> Vec<Segment> {
        self.segmentify(self.width, |x| self.is_col_black(x, col_height))
    }

    fn segmentify<F>(&self, max: usize, is_span_black: F) -> Vec<Segment> 
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


