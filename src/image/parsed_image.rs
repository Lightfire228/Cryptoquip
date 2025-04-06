use crate::image::Point;

use super::{iter_rect, raw_image::RawImage, Color, Rect};

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
    answer: Rect,
    clue:   Rect,
}

impl ParsedImage {
    pub fn new(raw_image: RawImage, boxes: Boxes) -> Self {

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
            let last_row  = boxes.last ().unwrap();

            let first_row = get_rect(&raw_image, first_row);
            let last_row  = get_rect(&raw_image, last_row);

            let (puzzle, answer) = split_answer(
                &raw_image,
                &boxes[1 .. &boxes.len() -1]
            )?;

    
            Ok(ParsedImage { 
                image:  raw_image,
    
                header: first_row,
                puzzle,
                answer,
                clue:   last_row,
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

    pub fn _test(&mut self) {
        self.image.fill(&self.clue, Color::Black);
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


fn get_rect(image: &RawImage, row: &Vec<Rect>) -> Rect {

    assert!(row.len() > 0);

    let first_box = row.first().unwrap();
    let last_box  = row.last() .unwrap();

    let x_start = 0;
    let x_end   = image.width;

    let y_start = first_box.top_left    .y;
    let y_end   = last_box .bottom_right.y;

    Rect {
        top_left:     Point { x: x_start, y: y_start },
        bottom_right: Point { x: x_end,   y: y_end   },
    }
}

fn split_answer(image: &RawImage, data: &[Vec<Rect>]) -> ImageParseResult<(Rect, Rect)> {

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

        let first = get_rect(image, first);
        let last  = get_rect(image, last);

        Rect {
            top_left:     first.top_left,
            bottom_right: last.bottom_right,
        }
    };
    
    let (puzzle, answer) = data.split_at(answer_ind);

    let puzzle = get(puzzle);
    let answer = get(answer);

    Ok((puzzle, answer))
}