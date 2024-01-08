use ash::vk;

use crate::device::Device;

#[derive(Clone)]
pub struct Framebuffer{
    framebuffer:vk::Framebuffer,
    extent:vk::Extent2D
}

impl Framebuffer{
    pub fn new(device:&Device,attachments: [vk::ImageView; 2],render_pass:vk::RenderPass,width:u32,height:u32) -> Self{
        let framebuffer_info = vk::FramebufferCreateInfo {
            s_type: vk::StructureType::FRAMEBUFFER_CREATE_INFO,
            render_pass: render_pass,
            p_next: std::ptr::null(),
            flags: vk::FramebufferCreateFlags::empty(),
            attachment_count: attachments.len() as u32,
            p_attachments: attachments.as_ptr(),
            width: width,
            height: height,
            layers: 1,
        };

        unsafe{
            let framebuffer = device.device().create_framebuffer(&framebuffer_info, None).expect("Failed to create framebuffer!");
            let extent = vk::Extent2D{width,height};
            Self{
                framebuffer,
                extent
            }
        }
    }

    pub fn get_framebuffer(&self) -> vk::Framebuffer{
        return self.framebuffer;
    }

    pub fn get_extent(&self) -> vk::Extent2D{
        return self.extent;
    }

    pub fn clean_framebuffer(&mut self,device:&Device){
        unsafe{
            device.device().destroy_framebuffer(self.framebuffer, None);
        }
    }
}