use pdf::object::ImageDict;

use super::{Color, Pixel, Point, Rect, Segment};


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
    *pix < 255
}


struct RectIter<'a> {
    rect: &'a Rect,

    x: usize,
    y: usize,
}

impl<'a> Iterator for RectIter<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {

        if self.x >= self.rect.bottom_right.x {
            self.x  = self.rect.top_left.x;
            self.y += 1
        }

        if self.y >= self.rect.bottom_right.y {
            return None;
        }

        let (x, y) = (self.x, self.y);


        self.x += 1;
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
