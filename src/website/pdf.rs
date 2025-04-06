use pdf::{backend::Backend, file::FileOptions, object::{PageRc, Resolve, XObject}};
use scraper::Html;
use std::{ops::Deref, vec};
use crate::image::RawImage;

use super::{
    get_selector,
    select_first,
    get_attr,
    ImageContext,
    request::{get_image_page, get_pdf}, 
};

type ParseResult<T> = Result<T, ParseErrorType>;

use ParseErrorType::*;


pub fn download_pdf_binary(ctx: &ImageContext) -> RawImage {

    match (|| {
        let page      = get_image_page(&ctx);
        let pdf_url   = extract_pdf_url(&page)?;
        let pdf_bytes = get_pdf(&pdf_url);

        #[cfg(feature = "cache")]
        write_cache(&pdf_bytes);

        let image = extract_img(pdf_bytes)?;

        Ok(image)

    })() {
        Ok(x)    => x,
        Err(err) => display_error(err)
    }
}

#[cfg(feature = "cache")]
pub fn from_cache(data: Vec<u8>) -> RawImage {

    match (|| {
        let image = extract_img(data)?;

        Ok(image)

    })() {
        Ok(x)    => x,
        Err(err) => display_error(err)
    }
}

enum ParseErrorType {
    ContentNotFound,
    AnchorNotFound,
    HrefAttrNotFound,
    UnableToReadPDF,
    UnableToExtractImage,
    NoImagesFound,
}


fn display_error(err: ParseErrorType) -> ! {
    eprintln!("Error while trying to download PDF");

    match err {
        ContentNotFound       => panic!("Cannot find main content body"),
        AnchorNotFound        => panic!("Cannot find anchor tag"),
        HrefAttrNotFound      => panic!("Cannot find url from anchor tag"),
        UnableToReadPDF       => panic!("Error while attempting to read PDF"),
        UnableToExtractImage  => panic!("Error while attempting to extract the image from the PDF"),
        NoImagesFound         => panic!("No images were found in the PDF"),
    }
}

fn extract_pdf_url(page: &str) -> ParseResult<String> {
    let content_selector = get_selector("#asset-content");
    let anchor_selector  = get_selector("a");

    let document = Html::parse_document(page);
    let document = document.root_element();

    let content  = select_first(&document, &content_selector).ok_or(ContentNotFound) ?;
    let anchor   = select_first(&content,  &anchor_selector) .ok_or(AnchorNotFound)  ?;
    let href     = get_attr    (&anchor,   "href")           .ok_or(HrefAttrNotFound)?;

    Ok(href.to_owned())
}

#[cfg(feature = "cache")]
fn write_cache(bytes: &Vec<u8>) {
    use crate::cache;

    cache::write_cache(bytes.as_slice());
}

struct _Bytes(Vec<u8>);

// why must you be this way
impl Backend for _Bytes {
    fn read<T: pdf::backend::IndexRange>(&self, range: T) -> pdf::error::Result<&[u8]> {

        let len = self.0.len();

        let start = range.start().unwrap_or(0);
        let end   = range.start().unwrap_or(len);

        Ok(&self.0[start..end])
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

// https://github.com/omkar-mohanty/vortex/blob/main/src/extractor/mod.rs
fn extract_img(bytes: Vec<u8>) -> ParseResult<RawImage> {

    let file     = FileOptions::cached().load(bytes).map_err(|_| UnableToReadPDF)?;
    let resolver = file.resolver();

    let mut images = vec![];
    let     pages  = file
        .pages()
        .map(|p| p.map_err(|_| UnableToReadPDF))
        .collect::<ParseResult<Vec<PageRc>>>()
        ?
    ;

    for page in pages {
        let resources = page.resources().map_err(|_| UnableToReadPDF)?;

        images.extend(
            resources.xobjects
                .iter()
                .map(|(_, &r)| resolver.get(r).unwrap())
                .filter(|o| matches!(**o, pdf::object::XObject::Image(_)))
        );
    }

    for o in images {
        let img = match *o {
            XObject::Image(ref im) => im,
            _                      => continue,
        };

        let data     = img.image_data(&resolver).map_err(|_| UnableToExtractImage)?;
        let img_dict = img.deref().to_owned();

        return Ok(RawImage::from_pdf(data.to_vec(), img_dict));
    }

    Err(NoImagesFound)

}

