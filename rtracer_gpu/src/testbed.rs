use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::device::{Device, DeviceExtensions, Queue};
use vulkano::format::Format;
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, Subpass, RenderPassAbstract};
use vulkano::image::{SwapchainImage, ImmutableImage, StorageImage, Dimensions};
use vulkano::image::traits::ImageViewAccess;
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::vertex::TwoBuffersDefinition;
use vulkano::sampler::{Sampler, SamplerAddressMode, Filter, MipmapMode};
use vulkano::swapchain::{AcquireError, PresentMode, SurfaceTransform, Swapchain, SwapchainCreationError, Surface};
use vulkano::swapchain;
use vulkano::sync::{GpuFuture, FlushError};
use vulkano::sync;

use vulkano_win::VkSurfaceBuild;

use winit::{EventsLoop, Window, WindowBuilder, Event, WindowEvent, ElementState, VirtualKeyCode};

use image::ImageFormat;

use std::sync::Arc;

use rtracer_core::prelude::*;

pub struct Testbed {
    pub instance: Arc<Instance>,
    pub surface: Arc<Surface<Window>>,
    pub events_loop: EventsLoop,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
}

impl Testbed {
    pub fn new() -> Testbed {
        let extensions = vulkano_win::required_extensions();
        let instance = Instance::new(None, &extensions, None).unwrap();

        let physical = PhysicalDevice::enumerate(&instance).next().unwrap();
        println!("Using device: {} (type: {:?})", physical.name(), physical.ty());

        let mut events_loop = EventsLoop::new();
        let surface = WindowBuilder::new().build_vk_surface(&events_loop, instance.clone()).unwrap();
        let window = surface.window();

        let queue_family = physical.queue_families().find(|&q|
            q.supports_graphics() && q.supports_compute() && surface.is_supported(q).unwrap_or(false)
        ).unwrap();

        let device_ext = DeviceExtensions { khr_swapchain: true, .. DeviceExtensions::none() };
        let (device, mut queues) = Device::new(physical, physical.supported_features(), &device_ext,
                                               [(queue_family, 0.5)].iter().cloned()).unwrap();
        let queue = queues.next().unwrap();

        Testbed { instance, surface, events_loop, device, queue }
    }

    pub fn run(&mut self, render_handler: &mut FnMut(Arc<dyn ImageViewAccess + Send + Sync>, Box<GpuFuture>) -> Box<GpuFuture>, event_handler: &mut FnMut(Event), end_frame_handler: &mut FnMut(),
               (width, height): (u32, u32)) {
        let instance = &self.instance;
        let surface = &self.surface;
        let events_loop = &mut self.events_loop;
        let device = &self.device;
        let queue = self.queue.clone();

        let physical = PhysicalDevice::enumerate(&instance).next().unwrap();

        let window = surface.window();

        let (mut swapchain, images) = {
            let caps = surface.capabilities(physical).unwrap();

            let usage = caps.supported_usage_flags;
            let alpha = caps.supported_composite_alpha.iter().next().unwrap();
            let format = caps.supported_formats[0].0;

            let initial_dimensions = if let Some(dimensions) = window.get_inner_size() {
                // convert to physical pixels
                let dimensions: (u32, u32) = dimensions.to_physical(window.get_hidpi_factor()).into();
                [dimensions.0, dimensions.1]
            } else {
                // The window no longer exists so exit the application.
                return;
            };

            Swapchain::new(device.clone(), surface.clone(), caps.min_image_count, format,
                           initial_dimensions, 1, usage, &queue, SurfaceTransform::Identity, alpha,
                           PresentMode::Fifo, true, None).unwrap()
        };

        #[derive(Debug, Clone)]
        struct Vertex { position: [f32; 2], texture_coords: [f32; 2] }
        vulkano::impl_vertex!(Vertex, position, texture_coords);

        let vertex_buffer = CpuAccessibleBuffer::<[Vertex]>::from_iter(
            device.clone(),
            BufferUsage::all(),
            [
                Vertex { position: [-1., -1. ], texture_coords: [0., 0.] },
                Vertex { position: [-1.,  1. ], texture_coords: [0., 1.] },
                Vertex { position: [ 1., -1. ], texture_coords: [1., 0.] },
                Vertex { position: [ 1.,  1. ], texture_coords: [1., 1.] },
            ].iter().cloned()
        ).unwrap();

        let vs = vs::Shader::load(device.clone()).unwrap();
        let fs = fs::Shader::load(device.clone()).unwrap();

        let render_pass = Arc::new(
            vulkano::single_pass_renderpass!(device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: swapchain.format(),
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        ).unwrap()
        );

        let pipeline = Arc::new(GraphicsPipeline::start()
            .vertex_input_single_buffer::<Vertex>()
            .vertex_shader(vs.main_entry_point(), ())
            .triangle_strip()
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(fs.main_entry_point(), ())
            .blend_alpha_blending()
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .build(device.clone())
            .unwrap());

        let texture = StorageImage::new(device.clone(),
                                        Dimensions::Dim2d { width, height },
                                        Format::R8G8B8A8Unorm, Some(queue.family())).unwrap();

        let sampler = Sampler::new(device.clone(), Filter::Linear, Filter::Linear,
                                   MipmapMode::Nearest, SamplerAddressMode::Repeat, SamplerAddressMode::Repeat,
                                   SamplerAddressMode::Repeat, 0.0, 1.0, 0.0, 0.0).unwrap();


        let set = Arc::new(PersistentDescriptorSet::start(pipeline.clone(), 0)
            .add_sampled_image(texture.clone(), sampler.clone()).unwrap()
            .build().unwrap()
        );

        let mut dynamic_state = DynamicState { line_width: None, viewports: None, scissors: None };
        let mut framebuffers = window_size_dependent_setup(&images, render_pass.clone(), &mut dynamic_state);

        let mut recreate_swapchain = false;
        let mut previous_frame_end = Box::new( vulkano::sync::now(device.clone())) as Box<GpuFuture>;

        loop {
            previous_frame_end.cleanup_finished();
            if recreate_swapchain {
                let dimensions = if let Some(dimensions) = window.get_inner_size() {
                    let dimensions: (u32, u32) = dimensions.to_physical(window.get_hidpi_factor()).into();
                    [dimensions.0, dimensions.1]
                } else {
                    return;
                };

                let (new_swapchain, new_images) = match swapchain.recreate_with_dimension(dimensions) {
                    Ok(r) => r,
                    Err(SwapchainCreationError::UnsupportedDimensions) => continue,
                    Err(err) => panic!("{:?}", err)
                };

                swapchain = new_swapchain;
                framebuffers = window_size_dependent_setup(&new_images, render_pass.clone(), &mut dynamic_state);

                recreate_swapchain = false;
            }

            let (image_num, future) = match swapchain::acquire_next_image(swapchain.clone(), None) {
                Ok(r) => r,
                Err(AcquireError::OutOfDate) => {
                    recreate_swapchain = true;
                    continue;
                }
                Err(err) => panic!("{:?}", err)
            };

            previous_frame_end = render_handler(texture.clone(), previous_frame_end);

            let clear_values = vec!([0.0, 0.0, 1.0, 1.0].into());
            let cb = AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family())
                .unwrap()
                .begin_render_pass(framebuffers[image_num].clone(), false, clear_values).unwrap()
                .draw(pipeline.clone(), &dynamic_state, vertex_buffer.clone(), set.clone(), ()).unwrap()
                .end_render_pass().unwrap()
                .build().unwrap();

            let future = previous_frame_end.join(future)
                .then_execute(queue.clone(), cb).unwrap()
                .then_swapchain_present(queue.clone(), swapchain.clone(), image_num)
                .then_signal_fence_and_flush();

            match future {
                Ok(future) => {
                    previous_frame_end = Box::new(future) as Box<_>;
                }
                Err(FlushError::OutOfDate) => {
                    recreate_swapchain = true;
                    previous_frame_end = Box::new(sync::now(device.clone())) as Box<_>;
                }
                Err(e) => {
                    println!("{:?}", e);
                    previous_frame_end = Box::new(sync::now(device.clone())) as Box<_>;
                }
            }

            let mut done = false;
            events_loop.poll_events(|ev| {
                match ev {
                    Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => done = true,
                    Event::WindowEvent { event: WindowEvent::Resized(_), .. } => recreate_swapchain = true,
                    _ => event_handler(ev)
                }
            });
            if done { return; }

            end_frame_handler();
        }
    }
}


/// This method is called once during initialization, then again whenever the window is resized
fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<RenderPassAbstract + Send + Sync>,
    dynamic_state: &mut DynamicState
) -> Vec<Arc<FramebufferAbstract + Send + Sync>> {
    let dimensions = images[0].dimensions();

    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [dimensions.width() as f32, dimensions.height() as f32],
        depth_range: 0.0 .. 1.0,
    };
    dynamic_state.viewports = Some(vec!(viewport));

    images.iter().map(|image| {
        Arc::new(
            Framebuffer::start(render_pass.clone())
                .add(image.clone()).unwrap()
                .build().unwrap()
        ) as Arc<FramebufferAbstract + Send + Sync>
    }).collect::<Vec<_>>()
}

mod vs {
    vulkano_shaders::shader!{
        ty: "vertex",
        src: "
#version 450

layout(location = 0) in vec2 position;
layout(location = 1) in vec2 texture_coords;
layout(location = 0) out vec2 tex_coords;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    tex_coords = texture_coords;
}"
    }
}

mod fs {
    vulkano_shaders::shader!{
        ty: "fragment",
        src: "
#version 450
layout(location = 0) in vec2 tex_coords;
layout(location = 0) out vec4 f_color;
layout(set = 0, binding = 0) uniform sampler2D tex;
void main() {
    f_color = texture(tex, tex_coords);
}"
    }
}
