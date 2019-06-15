use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::instance::{Instance, InstanceExtensions, PhysicalDevice};
use vulkano::device::{Device, DeviceExtensions};
use vulkano::format::{Format, ClearValue};
use vulkano::image::{StorageImage, Dimensions};
use vulkano::command_buffer::{CommandBuffer, AutoCommandBufferBuilder};
use vulkano::sync::GpuFuture;

use image::{ImageBuffer, Rgba};

fn main() {
    let instance = Instance::new(None, &InstanceExtensions::none(), None).unwrap();
    let physical = PhysicalDevice::enumerate(&instance).next().unwrap();
    let queue_family = physical.queue_families().find(|&q| q.supports_compute()).unwrap();

    let (device, mut queues) = Device::new(physical, physical.supported_features(),
                                           &DeviceExtensions::none(), [(queue_family, 0.5)].iter().cloned()).unwrap();

    let queue = queues.next().unwrap();


    let (width, height) = (1024, 1024);
    let buf = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(),
                                             (0 .. width * height * 4).map(|_| 0u8)).unwrap();

    let image = StorageImage::new(device.clone(),
                                  Dimensions::Dim2d { width, height },
                                  Format::R8G8B8A8Unorm, Some(queue.family())).unwrap();

    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap()
        .clear_color_image(image.clone(), ClearValue::Float([0.0, 0.0, 1.0, 1.0])).unwrap()
        .copy_image_to_buffer(image.clone(), buf.clone()).unwrap()
        .build().unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished.then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();

    let buffer_content = buf.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();

    image.save("image.png").unwrap();
}
