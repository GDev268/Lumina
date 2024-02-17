use std::rc::Rc;

use ash::vk;
use image::{DynamicImage, Rgba, GenericImage};
use crate::{device::Device, ImageValue};

#[derive(Clone)]
pub struct Texture {
    texture_path: String,
    texture: DynamicImage,
}

impl Texture {
    pub fn new(texture_path: String) -> Self {
        let texture = image::open(&texture_path);

        if texture.is_err() {
            Self{
                texture_path,
                texture: Texture::create_missing_texture()
            }
        }
        else{
            if texture.as_ref().unwrap().color() != image::ColorType::Rgba8 {
                let texture_buffer = texture.unwrap().to_rgba8();
                let texture = DynamicImage::ImageRgba8(texture_buffer);

                Self{
                    texture_path,
                    texture
                } 
            } else{
                Self{
                    texture_path,
                    texture: texture.unwrap()
                } 
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

    pub fn get_texture_info(&self) -> (u32,u32,u32) {
        return (self.texture.width(),self.texture.height(),4 as u32);
    }

    pub fn get_texture_data(&self) -> Vec<u8>{
        return self.texture.clone().into_bytes();
    }
}

impl ImageValue for Texture {}
