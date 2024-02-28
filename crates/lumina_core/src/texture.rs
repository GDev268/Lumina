use std::{
    rc::Rc,
    sync::{Arc, RwLock},
};

use crate::{device::Device, ImageValue};
use ash::vk;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImage, Rgba};
use lumina_files::loader::Loader;
use lumina_path::Path;

#[derive(Debug,Clone)]
pub struct Texture {
    pub texture_path: Path,
    pub pack_id: u32,
}

impl Texture {
    pub fn new_raw(texture_path: &str) -> Self {
        let file_path = lumina_path::get_raw_image(texture_path);

        if file_path.is_none() {
            Self {
                texture_path: Path::default(),
                pack_id: 0,
            }
        } else {
            Self {
                texture_path: file_path.unwrap(),
                pack_id: 0,
            }
        }
    }

    pub fn new(texture_path: &str, loader: Arc<RwLock<Loader>>) -> Self {
        let mut file_path = Path::default();

        file_path = lumina_path::get_new_image(texture_path, Arc::clone(&loader)).unwrap();

        Self {
            texture_path: file_path,
            pack_id: 0,
        }
    }

    fn create_missing_texture(is_raw: bool) -> DynamicImage {
        let mut texture = if is_raw {
            let mut image = DynamicImage::new_rgba8(64, 64);

            let cube_size = 8;

            let cube_number = 64 / cube_size;

            for cube_y in 0..cube_number {
                for cube_x in 0..cube_number {
                    let color: Rgba<u8> = if (cube_x + cube_y) % 2 == 0 {
                        Rgba([0, 0, 0, 255]) // Red
                    } else {
                        Rgba([0, 0, 0, 255]) // Green
                    };

                    for y in 0..cube_size {
                        for x in 0..cube_size {
                            let pixel_x = cube_x * cube_size + x;
                            let pixel_y = cube_y * cube_size + y;
                            image.put_pixel(pixel_x, pixel_y, color);
                        }
                    }
                }
            }
            image
        } else {
            let mut image = DynamicImage::new_rgba8(64, 64);

            let cube_size = 8;

            let cube_number = 64 / cube_size;

            for cube_y in 0..cube_number {
                for cube_x in 0..cube_number {
                    for y in 0..cube_size {
                        for x in 0..cube_size {
                            let pixel_x = cube_x * cube_size + x;
                            let pixel_y = cube_y * cube_size + y;
                            image.put_pixel(pixel_x, pixel_y, Rgba([0,0,0,0]));
                        }
                    }
                }
            }
            image
        };

        texture
    }

    pub fn create_texture(&self) -> DynamicImage {
        if self.texture_path.is_raw_path() {
            if !self.texture_path.get_raw_path().is_empty() {
                let texture = image::open(self.texture_path.get_raw_path()).unwrap();

                if texture.color() != image::ColorType::Rgba8 {
                    let texture_buffer = texture.to_rgba8();
                    DynamicImage::ImageRgba8(texture_buffer)
                } else {
                    texture
                }
            } else {
                Texture::create_missing_texture(true)
            }
        } else {
            if !self.texture_path.get_new_path().is_empty() {
                let texture = image::open(self.texture_path.get_raw_path()).unwrap();

                if texture.color() != image::ColorType::Rgba8 {
                    let texture_buffer = texture.to_rgba8();
                    DynamicImage::ImageRgba8(texture_buffer)
                } else {
                    texture
                }
            } else {
                Texture::create_missing_texture(false)
            }
        }
    }

    pub fn get_texture_info(&self) -> (u32, u32) {
        if self.texture_path.is_raw_path() {
            if !self.texture_path.get_raw_path().is_empty() {
                ImageReader::open(self.texture_path.get_raw_path())
                    .unwrap()
                    .with_guessed_format()
                    .unwrap()
                    .into_dimensions()
                    .unwrap()
            } else {
                (64, 64)
            }
        } else {
            if !self.texture_path.get_new_path().is_empty() {
                ImageReader::open(self.texture_path.get_new_path())
                    .unwrap()
                    .with_guessed_format()
                    .unwrap()
                    .into_dimensions()
                    .unwrap()
            } else {
                (64, 64)
            }
        }
    }

    pub fn get_new_path(&self) -> String {
        self.texture_path.get_new_path().to_string()
    }

    pub fn get_raw_path(&self) -> String {
        self.texture_path.get_raw_path().to_string()
    }
}

impl ImageValue for Texture {}
