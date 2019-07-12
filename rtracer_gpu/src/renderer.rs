use std::sync::Arc;

use vulkano::device::{Device, Queue};
use vulkano::pipeline::{ComputePipeline, ComputePipelineAbstract};
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::sync::GpuFuture;
use vulkano::image::{ImageViewAccess, StorageImage, Dimensions};
use vulkano::format::Format;

use rtracer_core::prelude::*;
use rand::Rng;
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};

extern crate rand;

pub mod cs {
    vulkano_shaders::shader! {
                ty: "compute",
//                path: "src/shaders/spheres.comp",
                path: "src/shaders/primitive.comp",
    }
}

pub struct Renderer {
    device: Arc<Device>,
    queue: Arc<Queue>,
    compute_pipeline: Arc<dyn ComputePipelineAbstract + Send + Sync>,
}

impl Renderer {
    pub fn new(device: Arc<Device>, queue: Arc<Queue>) -> Renderer {
        let compute_pipeline = Arc::new({
            let shader = cs::Shader::load(device.clone()).unwrap();
            ComputePipeline::new(device.clone(), &shader.main_entry_point(), &()).unwrap()
        });

        Renderer { device, queue, compute_pipeline }
    }

    pub fn create_texture(device: Arc<Device>, queue: Arc<Queue>, (width, height): (u32, u32)) -> Arc<dyn ImageViewAccess + Sync + Send> {
        StorageImage::new(device, Dimensions::Dim2d { width, height },
                          Format::R8G8B8A8Unorm, Some(queue.family())).unwrap()
    }

    pub fn render(&self, scene: &SceneData, camera: &Camera, image: Arc<dyn ImageViewAccess + Send + Sync>, future: Box<GpuFuture>) -> Box<GpuFuture>
    {
        let primitives_buffer = {
            let buf = primitives_to_gpu_buf(scene.primitives_iter());
            CpuAccessibleBuffer::from_iter(self.device.clone(), BufferUsage::all(), buf.iter().cloned()).unwrap()
        };

        let materials_buffer = {
            let buf = materials_to_gpu_buf(scene.materials_iter());
            CpuAccessibleBuffer::from_iter(self.device.clone(), BufferUsage::all(), buf.iter().cloned()).unwrap()
        };

        let objects_buffer = {
            let buf = objects_to_gpu_buf(scene.objects_iter());
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
            origin: raycast_camera.origin.into(),
            upper_left: raycast_camera.upper_left.into(),
            horizontal: raycast_camera.horizontal.into(),
            vertical: raycast_camera.vertical.into(),
            seed: rand::thread_rng().gen(),
            objects_count: scene.objects_count() as u32,
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

fn objects_to_gpu_buf<'a>(os: impl ExactSizeIterator<Item=(&'a ObjectId, &'a SceneObject)>) -> Vec<[u32; 2]> {
    // !todo: tmp
    let n = os.len();

    let mut buf = vec![[0, 0]; n];

    for (idx, o) in os {
        buf[idx.0 as usize] = [o.primitive().0, o.material().0];
    }

    buf
}

fn primitives_to_gpu_buf<'a>(ps: impl ExactSizeIterator<Item=(&'a PrimitiveId, &'a Primitive)>) -> Vec<[f32; 12]> {
    // !todo: tmp
    let n = ps.len();

    let mut buf = vec![[0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.]; n];

    for (idx, p) in ps {
        buf[idx.0 as usize] = primitive_to_gpu(p);
    }

    buf
}

fn primitive_to_gpu(primitive: &Primitive) -> [f32; 12] {
    match primitive {
        Primitive::Sphere(s) => {
            sphere_to_gpu(s)
        },
        Primitive::Triangle(t) => {
            triangle_to_gpu(t)
        },
        Primitive::Cube(c) => {
            cube_to_gpu(c)
        },
        _ => panic!("unsupported primitive")
    }
}

fn sphere_to_gpu(s: &Sphere) -> [f32; 12] {
    [
        0., 0., 0., 1.,
        s.center[0], s.center[1], s.center[2], s.radius,
        0., 0., 0., 0.,
    ]
}

fn triangle_to_gpu(t: &Triangle) -> [f32; 12] {
    [
        t.v0[0], t.v0[1], t.v0[2], 2.,
        t.v1[0], t.v1[1], t.v1[2], 0.,
        t.v2[0], t.v2[1], t.v2[2], 0.,
    ]
}

fn cube_to_gpu(c: &Cube) -> [f32; 12] {
    [
        c.aabb().min[0], c.aabb().min[1], c.aabb().min[2], 3.,
        c.aabb().max[0], c.aabb().max[1], c.aabb().max[2], 0.,
        0., 0., 0., 0.,
    ]
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
