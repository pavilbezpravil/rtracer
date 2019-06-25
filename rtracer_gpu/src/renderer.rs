use std::sync::Arc;

use vulkano::device::{Device, Queue};
use vulkano::pipeline::{ComputePipeline, ComputePipelineAbstract};
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::sync::GpuFuture;
use vulkano::image::traits::ImageViewAccess;

use rtracer_core::prelude::*;
use rand::Rng;

extern crate rand;

pub mod cs {
    vulkano_shaders::shader! {
                ty: "compute",
                path: "src/shaders/spheres.comp",
    }
}

pub struct Renderer {
    device: Arc<Device>,
    queue: Arc<Queue>,
    compute_pipeline: Arc<dyn ComputePipelineAbstract + Send + Sync>
}

impl Renderer {
    pub fn new(device: Arc<Device>, queue: Arc<Queue>) -> Renderer {
        let compute_pipeline = Arc::new({
            let shader = cs::Shader::load(device.clone()).unwrap();
            ComputePipeline::new(device.clone(), &shader.main_entry_point(), &()).unwrap()
        });

        Renderer { device, queue, compute_pipeline }
    }

    pub fn render(&self, camera: &Camera, image: Arc<dyn ImageViewAccess + Send + Sync>, future: Box<GpuFuture>) -> Box<GpuFuture>
    {
        let set = Arc::new(PersistentDescriptorSet::start(self.compute_pipeline.clone(), 0)
            .add_image(image.clone()).unwrap()
            .build().unwrap()
        );

        let raycast_camera = RaycastCamera::from_camera(camera);

        let camera_push_constant = cs::ty::PushConstant {
            origin: raycast_camera.origin.as_array(),
            upper_left: raycast_camera.upper_left.as_array(),
            horizontal: raycast_camera.horizontal.as_array(),
            vertical: raycast_camera.vertical.as_array(),
            seed: rand::thread_rng().gen(),
            shape_count: 0,
            _dummy0: [1, 1, 1, 1],
            _dummy1: [1, 1, 1, 1],
            _dummy2: [1, 1, 1, 1],
        };

        let (x_groups, y_groups) = {
            let dim = image.dimensions();
            let width = dim.width();
            let height = dim.height();
            debug_assert!(width % 8 == 0);
            debug_assert!(height % 8 == 0);
            (width / 8, height / 8)
        };

        let command_buffer = AutoCommandBufferBuilder::new(self.device.clone(), self.queue.family()).unwrap()
            .dispatch([x_groups, y_groups, 1], self.compute_pipeline.clone(), set.clone(), camera_push_constant).unwrap()
            .build().unwrap();

        let future = future
            .then_execute(self.queue.clone(), command_buffer).unwrap()
            .then_signal_fence_and_flush().unwrap();

        Box::new(future) as Box<_>
    }
}
