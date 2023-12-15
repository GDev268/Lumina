use std::rc::Rc;

use ash::vk;
use image::{DynamicImage, Rgba, GenericImage};
use lumina_core::device::Device;
use lumina_data::buffer::Buffer;

struct Texture {
    device: Rc<Device>,
    texture_path: String,
    texture: DynamicImage,
    target: String,
    image_buffer: Buffer,
}

impl Texture {
    pub fn new(texture_path: String, target: String, device: Rc<Device>) -> Self {
        let texture = image::open(&texture_path);

        if !texture.is_err() {
            let buffer_size = (texture.as_ref().unwrap().width()
                * texture.as_ref().unwrap().height()
                * (texture.as_ref().unwrap().color()).channel_count() as u32)
                as u64;
            let image_buffer = Buffer::new(
                Rc::clone(&device),
                buffer_size,
                1,
                vk::BufferUsageFlags::TRANSFER_SRC,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            );

            Self {
                device,
                texture_path,
                texture: texture.unwrap(),
                target,
                image_buffer,
            }
        } else {
            let texture = Texture::create_missing_texture();

            let buffer_size = (texture.width()
                * texture.height()
                * (texture.color()).channel_count() as u32) as u64;
            let image_buffer = Buffer::new(
                Rc::clone(&device),
                buffer_size,
                1,
                vk::BufferUsageFlags::TRANSFER_SRC,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            );

            Self {
                device,
                texture_path,
                texture,
                target,
                image_buffer,
            }
        }
    }

    fn create_missing_texture() -> DynamicImage {
        let mut texture = DynamicImage::new_rgb8(64, 64);

        for cube_y in 0..16 {
            for cube_x in 0..16 {
                let color: Rgba<u8> = if (cube_x + cube_y) % 2 == 0 {
                    Rgba([255, 0, 0, 255]) // Red
                } else {
                    Rgba([0, 255, 0, 255]) // Green
                };

                for y in 0..4 {
                    for x in 0..4 {
                        let pixel_x = cube_x * 4 + x;
                        let pixel_y = cube_y * 4 + y;
                        texture.put_pixel(pixel_x, pixel_y, color);
                    }
                }
            }
        }
        texture
    }
}
