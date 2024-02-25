use std::collections::HashMap;

use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba};
use lumina_core::texture::Texture;

const MAX_IMAGES_PER_ROW: u32 = 5;
const PADDING: u32 = 5;
const SPACING: u32 = 5;

static mut CURRENT_ID: u32 = 0;

#[derive(Debug)]
pub struct ImageInfo {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
struct RowInfo {
    images: Vec<u32>,
    cur_x: u32,
    cur_y: u32,
    max_width: u32,
    max_height: u32,
}

pub struct Atlas {
    pub texture: DynamicImage,
    pub images: Vec<ImageInfo>,
    rows: Vec<RowInfo>,
    additive_height: u32,
    additive_width: u32,
    cur_row: u32,
}

impl Atlas {
    pub fn new() -> Self {
        let rows = vec![RowInfo {
            images: Vec::new(),
            cur_x: 0,
            cur_y: 0,
            max_width: 0,
            max_height: 0,
        }];


        Self {
            texture: DynamicImage::new_rgb8(0, 0),
            images: Vec::new(),
            additive_height: 0,
            additive_width: 0,
            cur_row: 0,
            rows,
        }
    }

    pub fn pack_textures(&mut self, values: Vec<&mut Texture>) {
        let mut image_cache: HashMap<u32,&mut Texture> = HashMap::new();

        for image in values {
            let image_id = self.create_new_image(image.get_texture_info());

            image_cache.insert(image_id, image);
        }

        println!("{:?}",self.rows);
        self.update_rows();

        if self.texture.width() < self.additive_width
            || self.texture.height() < self.additive_height
        {
            let mut new_image = DynamicImage::new_rgba8(self.additive_width, self.additive_height);

            for x in 0..self.texture.width() {
                for y in 0..self.texture.height() {
                    let pixel = self.texture.get_pixel(x, y);
                    new_image.put_pixel(x, y, pixel);
                }
            }

            self.texture = new_image;
        }

        for (image_id, image) in image_cache {
            let image_info = self.images.get(image_id as usize).unwrap();
            let x_pos = image_info.x;
            let y_pos = image_info.y;


            let dyn_image = image.create_texture();
            for x in x_pos..x_pos + image_info.width {
                for y in y_pos..y_pos + image_info.height {
                    let pixel = dyn_image.get_pixel(x - x_pos, y - y_pos);
                    self.texture.put_pixel(x, y, pixel);
                }
            }
        }
    }

    pub fn pack_from_bytes(
        &mut self,
        data: Vec<Vec<[u8; 4]>>,
        width: u32,
        height: u32,
        texture: Vec<&mut Texture>,
    ) {
        let mut image_cache: HashMap<u32, DynamicImage> = HashMap::new();

        for pixels in data {
            let mut buffer: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);

            for pixel in pixels {
                buffer.push(pixel[0]);
                buffer.push(pixel[1]);
                buffer.push(pixel[2]);
                buffer.push(pixel[3]);
            }

            let image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
                ImageBuffer::from_vec(width, height, buffer).expect("Failed to create ImageBuffer");

            let dyn_image = DynamicImage::ImageRgba8(image_buffer);

            let image_id = self.create_new_image((width,height));

            image_cache.insert(image_id, dyn_image);
    
        }

        self.update_rows();

        if self.texture.width() < self.additive_width
            || self.texture.height() < self.additive_height
        {
            let mut new_image = DynamicImage::new_rgba8(self.additive_width, self.additive_height);

            for x in 0..self.texture.width() {
                for y in 0..self.texture.height() {
                    let pixel = self.texture.get_pixel(x, y);
                    new_image.put_pixel(x, y, pixel);
                }
            }

            self.texture = new_image;
        }

        for (image_id, image) in image_cache {
            let image_info = self.images.get(image_id as usize).unwrap();
            let x_pos = image_info.x;
            let y_pos = image_info.y;

            for x in x_pos..x_pos + image_info.width {
                for y in y_pos..y_pos + image_info.height {
                    let pixel = image.get_pixel(x - x_pos, y - y_pos);

                    self.texture.put_pixel(x, y, pixel);
                }
            }
        }
    }

    fn create_new_image(&mut self, size:(u32,u32)) -> u32 {
        let id = unsafe { CURRENT_ID };
        let image = ImageInfo {
            id: unsafe { CURRENT_ID },
            x: self.rows[self.cur_row as usize].cur_x + SPACING,
            y: self.rows[self.cur_row as usize].cur_y,
            width: size.0,
            height: size.1,
        };
        
        unsafe { CURRENT_ID += 1 };

        if self.rows[self.cur_row as usize].images.len() < MAX_IMAGES_PER_ROW as usize {
            self.rows[self.cur_row as usize].cur_x = image.x + image.width;
            self.rows[self.cur_row as usize].images.push(image.id);
            self.images.push(image);
        } else {
            self.update_rows();
            self.change_row(
                self.rows[self.cur_row as usize].max_height,
                self.rows[self.cur_row as usize].cur_y,
            );
            self.rows[self.cur_row as usize].images.push(image.id);
            self.images.push(image);
        }

        id
    }

    pub fn update_rows(&mut self) {
        self.additive_height = 0;
        self.additive_width = 0;

        for row in self.rows.iter_mut() {
            let mut max_width = 0;
            let mut max_height = 0;

            max_width = 0;
            max_height = 0;
            for image in row.images.iter_mut() {
                max_width += self.images[*image as usize].width + SPACING;

                if max_height < self.images[*image as usize].height {
                    max_height = self.images[*image as usize].height;
                }
            }

            row.max_width = max_width;
            row.max_height = max_height;

            if self.additive_width < row.max_width {
                self.additive_width = max_width;
            }

            self.additive_height += max_height + PADDING;

            println!("{:?}x{:?}",self.additive_width,self.additive_height);
        }
    }

    pub fn change_row(&mut self, prev_max_height: u32, prev_y: u32) {
        self.cur_row += 1;

        self.rows.push(RowInfo {
            images: Vec::new(),
            cur_x: 0,
            cur_y: prev_y + prev_max_height + PADDING,
            max_width: 0,
            max_height: 0,
        })
    }

    pub fn get_texture_data(&self) -> Vec<u8> {
        self.texture.clone().into_bytes()
    }

    pub fn get_texture_info(&self) -> (u32, u32, u32) {
        return (self.texture.width(), self.texture.height(), 4 as u32);
    }
}
