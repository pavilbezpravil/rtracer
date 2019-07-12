use std::cell::RefCell;

use winit::{Event, WindowEvent, ElementState, VirtualKeyCode};

use rtracer_core::prelude::*;

use rtracer_gpu::testbed::Testbed;
use rtracer_gpu::frame_counter::FrameCounter;
use rtracer_gpu::renderer::Renderer;
use std::alloc::handle_alloc_error;
use vulkano::sync::GpuFuture;

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

    scene
}

fn main() {
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

    let renderer = Renderer::new(testbed.device.clone(), testbed.queue.clone(), scene);

    let mut prev_frame_future = Box::new(vulkano::sync::now(device.clone())) as Box<dyn GpuFuture>;

    let texture = Renderer::create_texture(device.clone(), queue.clone(), (width, height));

    while !testbed.should_close() {
        prev_frame_future = testbed.prepare_frame(prev_frame_future).unwrap();

        let future = renderer.render(&camera, texture.clone(), prev_frame_future);

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
