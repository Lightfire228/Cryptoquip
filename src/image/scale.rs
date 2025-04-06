use crate::website::ImageContext;

use super::{Point, RawImage, Rect};

static LETTER_HEIGHT_IN: f64 = 11.0;
static LETTER_WIDTH_IN:  f64 =  8.5;

static MARGIN_IN:        f64 = 1.0;
static HEIGHT_IN:        f64 = 3.0;

static WIDTH_SUN_IN:     f64 = LETTER_WIDTH_IN - MARGIN_IN *2.0;


pub fn scale(image: &RawImage, ctx: &ImageContext) -> RawImage {
    let scale_factor = get_scale_factor(image, ctx);

    let mut scale = image.convert_to_letter_size(scale_factor);

    scale.copy_corners(image, scale_factor);

    scale
}


impl RawImage {
    fn convert_to_letter_size(&self, scale_factor: f64) -> Self {

        let (width, height) = self.get_letter_size(scale_factor);

        Self::new(width, height)
    }

    fn get_letter_size(&self, scale_factor: f64) -> (usize, usize) {

        let target_width  = (LETTER_WIDTH_IN  * scale_factor) as usize +1;
        let target_height = (LETTER_HEIGHT_IN * scale_factor) as usize +1;

        (target_width, target_height)

    }

    fn copy_corners(&mut self, source: &Self, scale_factor: f64) {
        let target_height = self.height;
        let source_height = source.height;
        let source_width  = source.width;

        let margin = (MARGIN_IN * scale_factor) as usize;

        let top_rect = Rect {
            top_left:     Point { x: margin,                y: margin },
            bottom_right: Point { x: margin + source_width, y: margin + source_height }
        };
        let bottom_rect = Rect {
            top_left:     Point { x: margin,                y: target_height - (margin + top_rect.get_height()) },
            bottom_right: Point { x: margin + source_width, y: target_height -  margin }
        };

        self.pixels_from(source, top_rect);
        self.pixels_from(source, bottom_rect);
    }

}

fn get_scale_factor(image: &RawImage, ctx: &ImageContext) -> f64 {
    let width  = image.width  as f64;
    let height = image.height as f64;

    if ctx.is_sunday() {
        width / WIDTH_SUN_IN
    }
    else {
        height / HEIGHT_IN
    }
}
