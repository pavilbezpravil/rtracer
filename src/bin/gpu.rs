use std::path::Path;

use rtracer_core::prelude::*;
use rtracer_core::model_loader::load_geometry_obj;

use rtracer_gpu::testbed::Testbed;
use rtracer_gpu::frame_counter::FrameCounter;
use rtracer_gpu::renderer::Renderer;
use rtracer_gpu::bvh::{Bvh, BvhItem};
use vulkano::sync::GpuFuture;
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::device::Device;
use std::sync::Arc;

fn create_scene() -> SceneData {
    let mut scene = SceneData::new();
    scene.create_object(Sphere::new(Vec3::new(-1., 0., -1.), 0.5).into(),
                        Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.0001).into());
    scene.create_object(Sphere::new(Vec3::new(0., 0., -1.5), 0.5).into(),
                        Metal::new(Vec3::new(0.2, 0.3, 0.34), 0.0001).into());
    scene.create_object(Sphere::new(Vec3::new(1., 0., -1.), 0.5).into(),
                        Metal::new(Vec3::new(0.45, 0.2, 0.4), 0.0001).into());
    scene.create_object(Sphere::new(Vec3::new(0., -100.5, -1.), 100.).into(),
                        Lambertian::new(Vec3::new(0.1, 0.8, 0.3)).into());

    let material = Metal::new(Vec3::new(0.45, 0.2, 0.4), 0.0001).into();
    scene.create_object(Triangle::new(Vec3::new(-1., 0., 0.), Vec3::new(0., 1., 0.), Vec3::new(1., 0., 0.)).into(), material);

//    add_triangles_to_scene(&mut scene, &load_some_obj(), material);

    scene.create_object(Cube::new(Vec3::new(0., 0., 0.), Vec3::new(2., 2., 2.)).into(),
                        Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.0001).into());

    scene
}

fn add_triangles_to_scene(scene: &mut SceneData, ts: &[Triangle], material: Material) {
    for t in ts {
        scene.create_object((*t).into(), material);
    }
}

fn load_some_obj() -> Vec<Triangle> {
    let path = "models/cube.obj";
//    let path = "models/sponza.obj";
    load_geometry_obj(Path::new(path)).unwrap()
}

fn object_to_bvh_item(scene: &SceneData, o: &SceneObject) -> BvhItem {
    let primitive_id = o.primitive();
    let primitive = scene.primitive(primitive_id).unwrap();
    let aabb = primitive.aabb();
    BvhItem { aabb, id: o.id().0 }
}

fn create_bvh(scene: &SceneData) -> Bvh {
    let mut bvh_items = vec![];

    for o in scene.objects_iter() {
        bvh_items.push(object_to_bvh_item(scene, o.1));
    }

    Bvh::build(&mut bvh_items)
}

fn create_bvh_nodes_buffer(device: Arc<Device>, bvh: &Bvh) -> Arc<CpuAccessibleBuffer<[f32]>> {
    let buf = bvh.to_gpu();
    CpuAccessibleBuffer::from_iter(device, BufferUsage::all(), buf.iter().cloned()).unwrap()
}

fn main() {

//    let a = 1242134;
//    let b = a as f32;
//    println!("{}", a);
//    println!("{}", b);
//
//    return;
    let (width, height) = (1920 / 2, 1080 / 2);
//    let (width, height) = (1920, 1080);
    // make size be devided by 8
    let (width, height) = ((width / 8) * 8, (height / 8) * 8);

    let mut camera = Camera::new(Vec3::z(), -Vec3::z(), Vec3::y(), 90., width as f32 / height as f32);
    let mut frame_counter = FrameCounter::new();

    let mut testbed = Testbed::new();
    testbed.init();

    let device = testbed.device.clone();
    let queue = testbed.queue.clone();

    let scene = create_scene();

    let renderer = Renderer::new(testbed.device.clone(), testbed.queue.clone());

    let mut prev_frame_future = Box::new(vulkano::sync::now(device.clone())) as Box<dyn GpuFuture>;

    let texture = Renderer::create_texture(device.clone(), queue.clone(), (width, height));

    let bvh = create_bvh(&scene);
    let bvh_nodes_buffer = create_bvh_nodes_buffer(device.clone(), &bvh);

    while !testbed.should_close() {
        prev_frame_future = testbed.prepare_frame(prev_frame_future).unwrap();

        let future = renderer.render(&scene, bvh_nodes_buffer.clone(), &camera, texture.clone(), prev_frame_future);

        prev_frame_future = testbed.render(future, texture.clone());

        testbed.handle_events(&mut |ev| {
            camera.process_input(&ev);
        });

        if let Some(fps) = frame_counter.next_frame() {
            println!("fps: {}", fps);
        }

        let dt = frame_counter.time_last_frame();
        camera.update(dt);
    }
}
