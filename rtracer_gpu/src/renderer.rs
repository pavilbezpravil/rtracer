use std::sync::Arc;

use vulkano::device::{Device, Queue};
use vulkano::pipeline::{ComputePipeline, ComputePipelineAbstract};
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::sync::GpuFuture;
use vulkano::image::traits::ImageViewAccess;

use rtracer_core::prelude::*;
use rand::Rng;
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};

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
    compute_pipeline: Arc<dyn ComputePipelineAbstract + Send + Sync>,
    scene: SceneData,
}

impl Renderer {
    pub fn new(device: Arc<Device>, queue: Arc<Queue>, scene: SceneData) -> Renderer {
        let compute_pipeline = Arc::new({
            let shader = cs::Shader::load(device.clone()).unwrap();
            ComputePipeline::new(device.clone(), &shader.main_entry_point(), &()).unwrap()
        });

        Renderer { device, queue, compute_pipeline, scene }
    }

    pub fn render(&self, camera: &Camera, image: Arc<dyn ImageViewAccess + Send + Sync>, future: Box<GpuFuture>) -> Box<GpuFuture>
    {
        let primitives_buffer = {
            let buf = primitives_to_gpu_buf(self.scene.primitives_iter());
            CpuAccessibleBuffer::from_iter(self.device.clone(), BufferUsage::all(), buf.iter().cloned()).unwrap()
        };

        let materials_buffer = {
            let buf = materials_to_gpu_buf(self.scene.materials_iter());
            CpuAccessibleBuffer::from_iter(self.device.clone(), BufferUsage::all(), buf.iter().cloned()).unwrap()
        };

        let objects_buffer = {
            let buf = objects_to_gpu_buf(self.scene.objects_iter());
            CpuAccessibleBuffer::from_iter(self.device.clone(), BufferUsage::all(), buf.iter().cloned()).unwrap()
        };

        let set = Arc::new(PersistentDescriptorSet::start(self.compute_pipeline.clone(), 0)
            .add_image(image.clone()).unwrap()
            .add_buffer(primitives_buffer).unwrap()
            .add_buffer(materials_buffer).unwrap()
            .add_buffer(objects_buffer).unwrap()
            .build().unwrap()
        );

        let raycast_camera = RaycastCamera::from_camera(camera);

        let camera_push_constant = cs::ty::PushConstant {
            origin: raycast_camera.origin.as_array(),
            upper_left: raycast_camera.upper_left.as_array(),
            horizontal: raycast_camera.horizontal.as_array(),
            vertical: raycast_camera.vertical.as_array(),
            seed: rand::thread_rng().gen(),
            objects_count: self.scene.objects_count() as u32,
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

fn objects_to_gpu_buf<'a>(os: impl ExactSizeIterator<Item=(&'a ObjectId, &'a Object)>) -> Vec<[u32; 2]> {
    // !todo: tmp
    let n = os.len();

    let mut buf = vec![[0, 0]; n];

    for (idx, o) in os {
        buf[idx.0 as usize] = [o.primitive().0, o.material().0];
    }

    buf
}

fn primitives_to_gpu_buf<'a>(ps: impl ExactSizeIterator<Item=(&'a PrimitiveId, &'a Primitive)>) -> Vec<[f32; 4]> {
    // !todo: tmp
    let n = ps.len();

    let mut buf = vec![[0., 0., 0., 0.]; n];

    for (idx, p) in ps {
        buf[idx.0 as usize] = primitive_to_gpu(p);
    }

    buf
}

fn primitive_to_gpu(primitive: &Primitive) -> [f32; 4] {
    match primitive {
        Primitive::Sphere(s) => {
            sphere_to_gpu(s)
        },
        _ => panic!("unsupported primitive")
    }
}

fn sphere_to_gpu(s: &Sphere) -> [f32; 4] {
    [s.center[0], s.center[1], s.center[2], s.radius]
}

fn materials_to_gpu_buf<'a>(ms: impl ExactSizeIterator<Item=(&'a MaterialId, &'a Material)>) -> Vec<[f32; 8]> {
    // !todo: tmp
    let n = ms.len();

    let mut buf = vec![[0., 0., 0., 0., 0., 0., 0., 0.]; n];

    for (idx, m) in ms {
        buf[idx.0 as usize] = material_to_gpu(m);
    }

    buf
}

fn material_to_gpu(m: &Material) -> [f32; 8] {
    match m {
        Material::Lambertian(l) => {
            lambertian_to_gpu(l)
        },
        Material::Metal(m) => {
            metal_to_gpu(m)
        },
        _ => panic!("unsupported material")
    }
}

const LAMBERTIAN: u32 = 1;
const METAL: u32 = 2;
const DIELECTRIC: u32 = 3;

fn lambertian_to_gpu(l: &Lambertian) -> [f32; 8] {
    [
        l.albedo[0], l.albedo[1], l.albedo[2], 0.,
        0., 0., 0., LAMBERTIAN as f32,
    ]
}

fn metal_to_gpu(m: &Metal) -> [f32; 8] {
    [
        m.albedo[0], m.albedo[1], m.albedo[2], m.fuzz,
        0., 0., 0., METAL as f32,
    ]
}
