use pdf::object::ImageDict;


pub struct RawImage {
    pub data:     Vec<u8>,
    pub img_dict: ImageDict,
}

