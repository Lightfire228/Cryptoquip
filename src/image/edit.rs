use crate::website::ImageContext;
use super::{parsed_image::ParsedImage, Color, RawImage, Rect};
use EditErrorType::*;

type EditResult<T> = Result<T, EditErrorType>;
type Boxes = Vec<Vec<Rect>>;




pub fn edit_image(mut img: RawImage, _ctx: &ImageContext) -> RawImage {
    match (|| {

        let boxes = img.rectangulate();

        img.hide_date(&boxes)?;
        
        // TODO: sunday
        let mut parsed_image = ParsedImage::new(img, boxes);

        Ok(parsed_image.clone_raw())
    })() {
        Ok(x)    => x,
        Err(err) => display_error(err)
    }
}


enum EditErrorType {
    FirstRowNotFound,
}

fn display_error(err: EditErrorType) -> ! {
    eprintln!("Error during image editing");

    match err {
        FirstRowNotFound => panic!("Unable to find first row of boxes"),
    }
}

impl RawImage {
    fn hide_date(&mut self, boxes: &Boxes) -> EditResult<()> {
    
        let first_row = boxes.first().ok_or(FirstRowNotFound)?;
        
        let w_quarter = self.width / 4 as usize;

        let date_boxes = first_row.iter().filter(|b| b.top_left.x < w_quarter);

        for b in date_boxes {
            self.fill(&b, Color::White);
        }
    
        Ok(())
    }
}
