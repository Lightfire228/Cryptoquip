use std::{env, fs::File, io::BufWriter, path::{Path, PathBuf}, process::Command};
use chrono::Local;
use png::Encoder;

use crate::image::RawImage;
use DisplayErrorType::*;

type DisplayResult<T> = Result<T, DisplayErrorType>;

pub fn display(image: &RawImage) {
    match (|| {
        let file = get_file_name();
        write_file(image, &file)?;

        display_platform(&file);

        Ok(())
    })() {
        Ok(x) => x,
        Err(err) => display_error(err)
    }
}

enum DisplayErrorType {
    UnableToCreateTempFile,
    UnableToWritePngData,
    UnableToLaunchImageWindow,
}

fn display_error(err: DisplayErrorType) -> ! {
    match err {
        UnableToCreateTempFile    => panic!("Unable to create a temp file to save image"),
        UnableToWritePngData      => panic!("Unable to write image data to temp file"),
        UnableToLaunchImageWindow => panic!("Unable to open image in new window"),
    }
}

fn write_file(raw_image: &RawImage, path: &Path) -> DisplayResult<()> {

    let file = File::create(path.to_owned()).map_err(|_| UnableToCreateTempFile)?;

    let width  = raw_image.width  as u32;
    let height = raw_image.height as u32;

    let ref mut w       = BufWriter::new(file);
    let     mut encoder = Encoder  ::new(w, width, height);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().map_err(|_| UnableToWritePngData)?;

    writer.write_image_data(&raw_image.data).map_err(|_| UnableToWritePngData)?;

    Ok(())
}

fn get_file_name() -> PathBuf {
    let temp_dir = env::temp_dir();

    let filename = Local::now()
        .to_rfc3339()
        .replace("-", "_")
        .replace("T", "_")
        .replace(":", "_")
    ;

    let filename = format!("cryptoquip_{}.png", filename);
    temp_dir.join(filename)
}

fn display_platform(path: &Path) {
    if cfg!(target_os = "windows") {
        display_windows(path);
    }
    else {
        display_linux(path);
    }
}

fn display_windows(path: &Path) -> DisplayResult<()> {
    Command::new("cmd")
        .args(&["/C", "start", path.to_str().unwrap()])
        .output()
        .map_err(|_| UnableToLaunchImageWindow)
        ?
    ;

    Ok(())
}

fn display_linux(path: &Path) -> DisplayResult<()> {
    Command::new("xdg-open")
        .args(&[path.to_str().unwrap()])
        .output()
        .map_err(|_| UnableToLaunchImageWindow)
        ?
    ;

    Ok(())
}
