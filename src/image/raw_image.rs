use pdf::object::ImageDict;

use super::{iter_rect, Color, Pixel, Point, Rect, Segment};


#[derive(Debug, Clone)]
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

    pub fn fill(&mut self, rect: &Rect, color: Color) {
        let color = match color {
            Color::White => 255,
            Color::Black =>   0,
        };
        
        for p in iter_rect(rect) {
            let pix = self.get_mut(p.x, p.y);

            *pix = color;
        }
    }

    pub fn pixels_from(&mut self, source: &RawImage, rect: Rect, y_start: usize) {

        for point in iter_rect(&rect) {
            let x        = point.x;
            let y        = point.y;
            let target_y = y_start + (point.y - rect.top_left.y);

            let source_pix = source.get    (x, y);
            let dest_pix   = self  .get_mut(x, target_y);

            *dest_pix = *source_pix;
        }
    }


    pub fn get_ind(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }

    pub fn get(&self, x: usize, y: usize) -> &Pixel {
        &self.data[self.get_ind(x, y)]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Pixel {
        let ind = self.get_ind(x, y);
        &mut self.data[ind]
    }

}


fn is_black(pix: &Pixel) -> bool {
    *pix < 255
}

