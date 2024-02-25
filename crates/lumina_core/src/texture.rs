use std::rc::Rc;

use crate::{device::Device, ImageValue};
use ash::vk;
use image::{DynamicImage, GenericImage, Rgba};
use lumina_path::Path;

#[derive(Clone)]
pub struct Texture {
    texture_path: Path,
    pub texture: Option<DynamicImage>,
    pub pack_id: u32,
}

impl Texture {
    pub fn new(texture_path: &str) -> Self {
        let file_path = lumina_path::get_raw_image(texture_path);

        if file_path.is_none() {
            Self {
                texture_path: Path::default(),
                texture: None,
                pack_id: 0,
            }
        } else {
            Self {
                texture_path: file_path.unwrap(),
                texture: None,
                pack_id: 0,
            }
        }
    }

    fn create_missing_texture() -> DynamicImage {
        let mut texture = DynamicImage::new_rgba8(64, 64);

        let cube_size = 8;

        let cube_number = 64 / cube_size;

        for cube_y in 0..cube_number {
            for cube_x in 0..cube_number {
                let color: Rgba<u8> = if (cube_x + cube_y) % 2 == 0 {
                    Rgba([255, 0, 255, 255]) // Red
                } else {
                    Rgba([0, 0, 0, 255]) // Green
                };

                for y in 0..cube_size {
                    for x in 0..cube_size {
                        let pixel_x = cube_x * cube_size + x;
                        let pixel_y = cube_y * cube_size + y;
                        texture.put_pixel(pixel_x, pixel_y, color);
                    }
                }
            }
        }
        texture
    }

    pub fn create_texture(&mut self) {
        if self.texture_path.get_raw_path() == "" {
            self.texture = Some(Texture::create_missing_texture());
        } else {
            let texture = image::open(self.texture_path.get_raw_path()).unwrap();

            if texture.color() != image::ColorType::Rgba8 {
                let texture_buffer = texture.to_rgba8();
                self.texture = Some(DynamicImage::ImageRgba8(texture_buffer));
            } else {
                self.texture = Some(texture);
            }
        }
    }

    pub fn get_texture_info(&self) -> (u32, u32, u32) {
        if self.texture.is_some() {
            return (
                self.texture.as_ref().unwrap().width(),
                self.texture.as_ref().unwrap().height(),
                4 as u32,
            );
        } else {
            return (0, 0, 0);
        }
    }

    pub fn get_texture_data(&self) -> Option<Vec<u8>> {
        if self.texture.is_some() {
            return Some(self.texture.as_ref().unwrap().clone().into_bytes());
        } else {
            return None;
        }
    }
}

impl ImageValue for Texture {}
