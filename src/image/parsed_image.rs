use super::{raw_image::{self, RawImage}, Rect};

type Boxes = Vec<Vec<Rect>>;

pub struct ParsedImage {
    image:  RawImage,

    header: Rect,
    puzzle: Rect,
    answer: Rect,
    clue:   Rect,
}

impl ParsedImage {
    pub fn new(raw_image: RawImage, boxes: Boxes) {

    }

    
}

