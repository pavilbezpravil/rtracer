use std::sync::Arc;

use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::instance::{Instance, InstanceExtensions, PhysicalDevice, PhysicalDeviceType};
use vulkano::device::{Device, DeviceExtensions};
use vulkano::format::Format;
use vulkano::image::{StorageImage, Dimensions};
use vulkano::command_buffer::{CommandBuffer, AutoCommandBufferBuilder};
use vulkano::sync::GpuFuture;

use image::{ImageBuffer, Rgba};

use rtracer_core::prelude::*;

use rtracer_gpu::renderer::Renderer;

fn print_all_physical_devices(instance: &Arc<Instance>) {
    for p in PhysicalDevice::enumerate(&instance) {
        println!("{}", p.name());
    }
}

fn physical_device_find_gpu_or_cpu(instance: &Arc<Instance>) -> Option<PhysicalDevice> {
    if let Some(p) = PhysicalDevice::enumerate(&instance).find(|p| p.ty() == PhysicalDeviceType::DiscreteGpu) {
        Some(p)
    } else if let Some(p) = PhysicalDevice::enumerate(&instance).find(|p| p.ty() == PhysicalDeviceType::IntegratedGpu) {
        Some(p)
    } else if let Some(p) = PhysicalDevice::enumerate(&instance).find(|p| p.ty() == PhysicalDeviceType::VirtualGpu) {
        Some(p)
    } else if let Some(p) = PhysicalDevice::enumerate(&instance).find(|p| p.ty() == PhysicalDeviceType::Cpu) {
        Some(p)
    } else {
        None
    }
}

fn main() {
    let instance = Instance::new(None, &InstanceExtensions::none(), None).unwrap();
    print_all_physical_devices(&instance);
    let physical = physical_device_find_gpu_or_cpu(&instance).unwrap();
    let queue_family = physical.queue_families().find(|&q| q.supports_compute()).unwrap();

    let (device, mut queues) = Device::new(physical, physical.supported_features(),
                                           &DeviceExtensions::none(), [(queue_family, 0.5)].iter().cloned()).unwrap();

    let queue = queues.next().unwrap();

    let (width, height) = (1024, 1024);
    let buf = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(),
                                             (0..width * height * 4).map(|_| 0u8)).unwrap();

    let image = StorageImage::new(device.clone(),
                                  Dimensions::Dim2d { width, height },
                                  Format::R8G8B8A8Unorm, Some(queue.family())).unwrap();

    let camera = Camera::new(Vec3::new_z(), -Vec3::new_z(), Vec3::new_y(), 90., width as f32 / height as f32);

    let renderer = Renderer::new(device.clone(), queue.clone());

    renderer.render(&camera, image.clone(), Box::new(vulkano::sync::now(device.clone())) as Box<GpuFuture>);

    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap()
        .copy_image_to_buffer(image.clone(), buf.clone()).unwrap()
        .build().unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished.then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();

    let buffer_content = buf.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();

    image.save("image.png").unwrap();
}
