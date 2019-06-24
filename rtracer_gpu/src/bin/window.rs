use std::cell::RefCell;

use winit::{Event, WindowEvent, ElementState, VirtualKeyCode};

use rtracer_core::prelude::*;

use rtracer_gpu::testbed::Testbed;
use rtracer_gpu::frame_counter::FrameCounter;
use rtracer_gpu::renderer::Renderer;

fn main() {
    let (width, height) = (1920, 1080);
    // make size be devided by 8
    let (width, height) = ((width / 8) * 8, (height / 8) * 8);

    let mut test_bed = Testbed::new();

    let camera = RefCell::new(Camera::new(Vec3::new_z(), -Vec3::new_z(), Vec3::new_y(), 90., width as f32 / height as f32));
    let renderer = Renderer::new(test_bed.device.clone(), test_bed.queue.clone());

    let mut frame_counter = FrameCounter::new();

    let mut render_handle = |image, future| {
        renderer.render(&camera.borrow(), image, future)
    };

    let mut event_handler = |ev| {
        match ev {
//                Event::WindowEvent { event: WindowEvent::CursorMoved { position, .. }, .. } => println!("{:?}", position),
            Event::WindowEvent { event: WindowEvent::KeyboardInput { input, .. }, .. } => {
                if input.state == ElementState::Pressed {
                    if let Some(key) = input.virtual_keycode {
                        let mut dir = Vec3::origin();
                        let mut camera = camera.borrow_mut();
                        match key {
                            VirtualKeyCode::W => {
                                dir += camera.forward();
                            },
                            VirtualKeyCode::S => {
                                dir += camera.backward();
                            },
                            VirtualKeyCode::A => {
                                dir += camera.left();
                            },
                            VirtualKeyCode::D => {
                                dir += camera.right();
                            },
                            _ => {},
                        };

                        if let Some(dir) = dir.try_make_unit() {
                            let dt = 1. / 60.;
                            camera.translate(&(dir * dt));
                        }
                    }
                }
            },
            _ => ()
        }
    };

    let mut end_loop_handler = || {
        if let Some(fps) = frame_counter.next_frame() {
            println!("fps: {}", fps);
        }
    };

    test_bed.run(&mut render_handle, &mut event_handler, &mut end_loop_handler, (width, height));
}
