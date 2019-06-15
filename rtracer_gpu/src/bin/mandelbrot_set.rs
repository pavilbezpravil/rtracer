use std::sync::Arc;

use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::instance::{Instance, InstanceExtensions, PhysicalDevice};
use vulkano::device::{Device, DeviceExtensions};
use vulkano::pipeline::ComputePipeline;
use vulkano::format::{Format, ClearValue};
use vulkano::image::{StorageImage, Dimensions};
use vulkano::command_buffer::{CommandBuffer, AutoCommandBufferBuilder};
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::sync::GpuFuture;

use image::{ImageBuffer, Rgba};

fn main() {
    let instance = Instance::new(None, &InstanceExtensions::none(), None).unwrap();
    let physical = PhysicalDevice::enumerate(&instance).next().unwrap();
    let queue_family = physical.queue_families().find(|&q| q.supports_compute()).unwrap();

    let (device, mut queues) = Device::new(physical, physical.supported_features(),
                                           &DeviceExtensions::none(), [(queue_family, 0.5)].iter().cloned()).unwrap();

    let queue = queues.next().unwrap();

    let compute_pipeline = Arc::new({
        mod cs {
            vulkano_shaders::shader! {
                ty: "compute",
                path: "src/shaders/mandelbrot_set.comp",
            }
        }
        let shader = cs::Shader::load(device.clone()).unwrap();
        ComputePipeline::new(device.clone(), &shader.main_entry_point(), &()).unwrap()
    });

    let (width, height) = (1024, 1024);
    let buf = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(),
                                             (0..width * height * 4).map(|_| 0u8)).unwrap();

    let image = StorageImage::new(device.clone(),
                                  Dimensions::Dim2d { width, height },
                                  Format::R8G8B8A8Unorm, Some(queue.family())).unwrap();

    let set = Arc::new(PersistentDescriptorSet::start(compute_pipeline.clone(), 0)
        .add_image(image.clone()).unwrap()
        .build().unwrap()
    );

    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap()
        .dispatch([1024 / 8, 1024 / 8, 1], compute_pipeline.clone(), set.clone(), ()).unwrap()
        .copy_image_to_buffer(image.clone(), buf.clone()).unwrap()
        .build().unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished.then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();

    let buffer_content = buf.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();

    image.save("image.png").unwrap();
}
