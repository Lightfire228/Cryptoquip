use crate::{image::Point, website::ImageContext};

use super::{raw_image::RawImage, Rect};

type Boxes = Vec<Vec<Rect>>;

pub enum ImageParseError {
    NotEnoughRows(usize),
    RowEmpty(usize),
    IndentedRowNotFound,
}

use ImageParseError::*;
type ImageParseResult<T> = Result<T, ImageParseError>;

static INDENT_THRESHOLD: usize = 10;
static PUZZLE_PADDING:   usize = 75;
static CLUE_PADDING:     usize = 50;

pub struct ParsedImage {
    pub image:  RawImage,

    header: Rect,
    puzzle: Rect,
    clue:   Rect,
}

impl ParsedImage {
    pub fn new(raw_image: RawImage, boxes: Boxes, ctx: &ImageContext) -> Self {

        match (|| {

            if boxes.len() < 4 {
                return Err(NotEnoughRows(boxes.len()));
            }

            for (i, row) in boxes.iter().enumerate() {
                if row.len() == 0 {
                    return Err(RowEmpty(i));
                }
            }

            let first_row = boxes.first().unwrap();
            let clue_row  = if ctx.is_sunday() {
                &boxes[boxes.len() -2]
            } 
            else {
                boxes.last ().unwrap()
            };

            let header    = raw_image.get_rect(first_row);
            let clue_rect = raw_image.get_rect(clue_row);

            let puzzle = raw_image.split_answer(
                &boxes[1 .. &boxes.len() -1]
            )?;

    
            Ok(ParsedImage { 
                image:  raw_image,
    
                header,
                puzzle,
                clue:   clue_rect,
            })

        })() {
            Ok(x)    => x,
            Err(err) => display_error(err)
        }
    }

    pub fn new_image_from_padding(&self) -> RawImage {
        let width  = self.image.width;

        let height = 0
            + self.header.get_height()
            + PUZZLE_PADDING
            + self.puzzle.get_height()
            + CLUE_PADDING
            + self.clue  .get_height()
        ;

        let size = height * width;

        let mut data = Vec::with_capacity(size);

        for _ in 0..size {
            data.push(255);
        }

        let mut image = RawImage { 
            data, 
            width, 
            height,
        };

        self.copy_regions(&mut image);

        image

    }

    fn copy_regions(&self, target: &mut RawImage) {
        let source = &self.image;

        let mut y = 0;

        target.pixels_from(source, self.header, y);
        y += self.header.get_height() + PUZZLE_PADDING;

        target.pixels_from(source, self.puzzle, y);
        y += self.puzzle.get_height() + CLUE_PADDING;

        target.pixels_from(source, self.clue,   y);
    }


}

fn display_error(err: ImageParseError) -> ! {

    eprintln!("Unable to parse image");

    match err {
        NotEnoughRows(len)  => panic!("Not enough rows were found: {} - Expected: 4", len),
        RowEmpty(x)         => panic!("Empty Row: {}", x),
        IndentedRowNotFound => panic!("Unable to find the Answer row")
    }
}

impl RawImage {

    fn get_rect(&self, row: &Vec<Rect>) -> Rect {

        assert!(row.len() > 0);

        let first_box = row.first().unwrap();
        let last_box  = row.last() .unwrap();

        let x_start = 0;
        let x_end   = self.width;

        let y_start = first_box.top_left    .y;
        let y_end   = last_box .bottom_right.y;

        Rect {
            top_left:     Point { x: x_start, y: y_start },
            bottom_right: Point { x: x_end,   y: y_end   },
        }
    }

    fn split_answer(&self, data: &[Vec<Rect>]) -> ImageParseResult<Rect> {

        assert!(data.len() >= 2);

        let mut answer_ind = None;

        for (i, row) in data.iter().enumerate().rev() {
            assert!(row.len() > 0);

            let first = row.first().unwrap();

            if first.top_left.x > INDENT_THRESHOLD {
                answer_ind = Some(i);
                break;
            }
        }

        let answer_ind = answer_ind.ok_or(IndentedRowNotFound)?;
        
        let get = |rows: &[Vec<Rect>]| {
            let first = rows.first().unwrap();
            let last  = rows.last ().unwrap();

            let first = self.get_rect(first);
            let last  = self.get_rect(last);

            Rect {
                top_left:     first.top_left,
                bottom_right: last.bottom_right,
            }
        };
        
        let (puzzle, _) = data.split_at(answer_ind);

        Ok(get(puzzle))
    }
}