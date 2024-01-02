use std::{cell::RefCell, ops::Deref, rc::Rc};

use lumina_bundle::RendererBundle;
use lumina_core::{
    device::Device,
    swapchain::{self, Swapchain, MAX_FRAMES_IN_FLIGHT},
    window::Window, framebuffer::Framebuffer, image::Image,
};

use ash::vk;
use lumina_graphic::pipeline::PipelineConfiguration;

use crate::canvas::{Canvas, self};

pub struct RenderTexture {
    pub images: Vec<Image>,
    pub depth_images: Vec<Image>,
    framebuffers: Vec<Framebuffer>,
    extent: vk::Extent2D,
}

pub struct Renderer {
    pub renderer_data: RenderTexture,
    camera_render_pass: vk::RenderPass,
    command_buffers: Vec<vk::CommandBuffer>,
    pub current_frame_index: usize,
    pub canvas:Canvas,
    in_flight_fences: Vec<vk::Fence>,
    images_in_flight: Vec<vk::Fence>,
}

impl Renderer {
    pub fn new(device: Rc<Device>, extent: vk::Extent2D, renderer_bundle: &RendererBundle) -> Self {

        let mut in_flight_fences = Vec::new();
        in_flight_fences.resize(MAX_FRAMES_IN_FLIGHT, vk::Fence::null());

        let mut images_in_flight = Vec::new();
        images_in_flight.resize(MAX_FRAMES_IN_FLIGHT, vk::Fence::null());

        let semaphore_info = vk::SemaphoreCreateInfo::default();

        let fence_info = vk::FenceCreateInfo {
            s_type: vk::StructureType::FENCE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::FenceCreateFlags::SIGNALED,
        };

        for i in 0..MAX_FRAMES_IN_FLIGHT {
            unsafe {
                in_flight_fences[i] = device.device().create_fence(&fence_info, None).unwrap();
            }
        }

        let mut canvas = Canvas::new(Rc::clone(&device));

        let mut pipeline_config = PipelineConfiguration::default();
        pipeline_config.attribute_descriptions = canvas.mesh.get_attribute_descriptions().clone();
        pipeline_config.binding_descriptions = canvas.mesh.get_binding_descriptions().clone();

        canvas.shader.create_pipeline_layout( false);
        canvas.shader.create_pipeline(renderer_bundle.render_pass,pipeline_config);

        Self {
            renderer_data: Renderer::create_renderer_data(&device, renderer_bundle, extent),
            command_buffers: Renderer::create_command_buffers(&device),
            current_frame_index: 0,
            in_flight_fences,
            images_in_flight,
            camera_render_pass: renderer_bundle.render_pass,
            canvas
        }
    }

    pub fn create_renderer_data(
        device: &Device,
        renderer_bundle: &RendererBundle,
        extent: vk::Extent2D,
    ) -> RenderTexture {
        let extent = if extent.width > renderer_bundle.max_extent.width
            || extent.height > renderer_bundle.max_extent.height
        {
            renderer_bundle.max_extent
        } else {
            extent
        };

        let mut renderer_data = RenderTexture {
            images: Vec::new(),
            depth_images: Vec::new(),
            framebuffers: Vec::new(),
            extent,
        };

        for i in 0..MAX_FRAMES_IN_FLIGHT {
            let mut image = Image::new_2d(
                device,
                renderer_bundle.image_format,
                vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
                extent.width,
                extent.height,
            );
            image.new_image_view(device, vk::ImageAspectFlags::COLOR);
            renderer_data.images.push(image);

            let mut depth_image = Image::new_2d(
                device,
                renderer_bundle.depth_format,
                vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
                extent.width,
                extent.height,
            );

            depth_image.new_image_view(device, vk::ImageAspectFlags::DEPTH);
            renderer_data.depth_images.push(depth_image);
        }

        for i in 0..MAX_FRAMES_IN_FLIGHT {
            let attachments = [
                renderer_data.images[i].get_image_view(),
                renderer_data.depth_images[i].get_image_view(),
            ];

            let framebuffer = Framebuffer::new(
                device,
                attachments,
                renderer_bundle.render_pass,
                extent.width,
                extent.height,
            );

            renderer_data.framebuffers.push(framebuffer);
        }

        renderer_data
    }

    pub fn create_command_buffers(device: &Device) -> Vec<vk::CommandBuffer> {
        let alloc_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: std::ptr::null(),
            level: vk::CommandBufferLevel::PRIMARY,
            command_pool: device.get_command_pool(),
            command_buffer_count: MAX_FRAMES_IN_FLIGHT as u32,
        };

        let command_buffers = unsafe {
            device
                .device()
                .allocate_command_buffers(&alloc_info)
                .expect("Failed to allocate command buffers!")
        };

        return command_buffers;
    }

    pub fn begin_frame(&mut self, device: &Device) {
        let command_buffer = self.get_command_buffer();

        let begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            p_next: std::ptr::null(),
            flags: vk::CommandBufferUsageFlags::empty(),
            p_inheritance_info: std::ptr::null(),
        };

        unsafe {
            device
                .device()
                .begin_command_buffer(command_buffer, &begin_info)
                .expect("Failed to begin recording command buffer");
        }

        self.begin_rendering(device, command_buffer);

    }

    pub fn end_frame(&mut self, device: &Device,wait_semaphore:vk::Semaphore) {
        let command_buffer = self.get_command_buffer();

        self.end_rendering(device, command_buffer);

        unsafe {
            device.device().end_command_buffer(command_buffer).unwrap();
        }

        self.submit_command_buffers(device, command_buffer, wait_semaphore,self.current_frame_index);
    }

    fn begin_rendering(&mut self, device: &Device, command_buffer: vk::CommandBuffer) {
        let mut clear_values: [vk::ClearValue; 2] =
            [vk::ClearValue::default(), vk::ClearValue::default()];

        clear_values[0].color = vk::ClearColorValue {
            float32: [0.1, 0.1, 0.0, 1.0],
        };
        clear_values[1].depth_stencil = vk::ClearDepthStencilValue {
            depth: 1.0,
            stencil: 0,
        };

        let renderpass_info = vk::RenderPassBeginInfo {
            s_type: vk::StructureType::RENDER_PASS_BEGIN_INFO,
            p_next: std::ptr::null(),
            render_pass: self.camera_render_pass,
            framebuffer: self.get_framebuffer(),
            render_area: vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: self.renderer_data.extent,
            },
            clear_value_count: clear_values.len() as u32,
            p_clear_values: clear_values.as_ptr(),
        };

        unsafe {
            device.device().cmd_begin_render_pass(
                command_buffer,
                &renderpass_info,
                vk::SubpassContents::INLINE,
            );
        }

        let viewport = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: self.renderer_data.extent.width as f32,
            height: self.renderer_data.extent.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        };

        let scissor = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: self.renderer_data.extent,
        };

        unsafe {
            device
                .device()
                .cmd_set_viewport(command_buffer, 0, &[viewport]);
            device
                .device()
                .cmd_set_scissor(command_buffer, 0, &[scissor]);
        }
    }

    fn end_rendering(&mut self, device: &Device, command_buffer: vk::CommandBuffer) {
        unsafe {
            device.device().cmd_end_render_pass(command_buffer);
        }
    }

    pub fn submit_command_buffers(
        &mut self,
        device: &Device,
        cmd_buffer: vk::CommandBuffer,
        wait_semaphore:vk::Semaphore,
        frame_index: usize,
    ) {
        if self.images_in_flight[frame_index] != vk::Fence::null() {
            unsafe {
                device
                    .device()
                    .wait_for_fences(&[self.images_in_flight[frame_index]], true, u64::MAX)
                    .unwrap();
            }
        }

        self.images_in_flight[frame_index] = self.in_flight_fences[frame_index];
        

        let submit_info = vk::SubmitInfo {
            s_type: vk::StructureType::SUBMIT_INFO,
            command_buffer_count: 1,
            p_command_buffers: &cmd_buffer,
            ..Default::default()
        };

        unsafe {
            device
                .device()
                .reset_fences(&[self.in_flight_fences[frame_index]])
                .unwrap();
            
            device
                .device()
                .queue_submit(
                    device.graphics_queue(),
                    &[submit_info],
                    self.in_flight_fences[frame_index],
                )
                .unwrap();

        }

        self.current_frame_index = (self.current_frame_index + 1) % MAX_FRAMES_IN_FLIGHT;
    }

    pub fn get_command_buffer(&self) -> vk::CommandBuffer {
        return self.command_buffers[self.current_frame_index];
    }

    pub fn get_framebuffer(&self) -> vk::Framebuffer {
        return self.renderer_data.framebuffers[self.current_frame_index].get_framebuffer();
    }

    fn free_command_buffers(&self, device: &Device) {
        unsafe {
            device
                .device()
                .free_command_buffers(device.get_command_pool(), &self.command_buffers);
        }
    }

    pub fn resize_camera_renderer(&mut self,renderer_bundle: &RendererBundle) {
        self.camera_render_pass = renderer_bundle.render_pass;
    }
}

/* pub fn render_game_objects(
        &mut self,
        device: &Device,
        frame_info: &FrameInfo,
        scene: &mut Query,
    ) {
        for (id, entity) in scene.entities.iter_mut() {
            if let Some(shader) = entity.get_mut_component::<Shader>() {
                if shader.pipeline_layout.is_none() && shader.pipeline.is_none() {
                    shader.pipeline_layout = Some(Renderer::create_pipeline_layout(
                        device,
                        shader
                            .descriptor_manager
                            .get_descriptor_layout()
                            .get_descriptor_set_layout(),
                    ));
                    shader.pipeline = Some(Renderer::create_pipeline(
                        self.get_swapchain_renderpass(),
                        shader,
                        device,
                    ));
                }

                shader
                    .pipeline
                    .as_ref()
                    .unwrap()
                    .bind(device, frame_info.command_buffer);

                unsafe {
                    device.device().cmd_bind_descriptor_sets(
                        frame_info.command_buffer,
                        vk::PipelineBindPoint::GRAPHICS,
                        shader.pipeline_layout.unwrap(),
                        0,
                        &[shader
                            .descriptor_manager
                            .get_descriptor_set(self.get_frame_index() as u32)],
                        &[],
                    );
                }
            };

            let push: PushConstantData = if entity.has_component::<Transform>() {
                PushConstantData {
                    model_matrix: entity.get_component::<Transform>().unwrap().get_mat4(),
                    normal_matrix: entity
                        .get_component::<Transform>()
                        .unwrap()
                        .get_normal_matrix(),
                }
            } else {
                PushConstantData {
                    model_matrix: glam::Mat4::default(),
                    normal_matrix: glam::Mat4::default(),
                }
            };

            let push_bytes: &[u8] = unsafe {
                let struct_ptr = &push as *const _ as *const u8;
                std::slice::from_raw_parts(struct_ptr, std::mem::size_of::<PushConstantData>())
            };

            if let Some(shader) = entity.get_component::<Shader>() {
                unsafe {
                    device.device().cmd_push_constants(
                        frame_info.command_buffer,
                        shader.pipeline_layout.unwrap(),
                        vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
                        0,
                        push_bytes,
                    );
                }

                if entity.has_component::<Model>() {
                    entity
                        .get_mut_component::<Model>()
                        .unwrap()
                        .render(device, frame_info.command_buffer);
                }
            }
        }
    }
*/
