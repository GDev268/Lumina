use image::DynamicImage;

#[derive(Debug)]
struct ImageInfo {
    x: u64,
    y: u64,
    width: u64,
    height: u64,
}

struct Atlas {
    texture: DynamicImage,
    images: Vec<Vec<ImageInfo>>,
    row_max_height: u32,
    image_quantity: u8,
}

impl Atlas {
    pub fn new() -> Self {
        Self {
            texture: DynamicImage::new_rgb8(0, 0),
            images: Vec::new(),
            row_max_height: 0,
            image_quantity: 0,
        }
    }

    //pub fn pack_textures(&mut self,)
}
