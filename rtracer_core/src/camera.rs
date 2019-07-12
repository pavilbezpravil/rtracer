use crate::{Vec3, Vec2};
use crate::ray::Ray;
use na::{Rotation3, Unit};

use winit::{Event, WindowEvent, ElementState, VirtualKeyCode};

pub struct RaycastCamera {
    pub origin: Vec3,
    pub upper_left: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl RaycastCamera {
    pub fn from_camera(camera: &Camera) -> RaycastCamera {
        let theta = camera.vfov.to_radians();
        let half_height = (theta / 2.).tan();
        let half_width = camera.aspect * half_height;

        let w = (camera.lookfrom - camera.lookat).normalize();
        let u = (camera.vup.cross(&w)).normalize();
        let v = (w.cross(&u)).normalize();

        let origin = camera.lookfrom;
        let upper_left = origin - half_width * u + half_height * v - w;
        let horizontal = 2. * half_width * u;
        let vertical = -2. * half_height * v;

        RaycastCamera { origin, upper_left, horizontal, vertical }
    }

    pub fn get_ray(&self, (u, v): (f32, f32)) -> Ray {
        debug_assert!(u >= 0f32 && u < 1.05f32);
        debug_assert!(v >= 0f32 && v < 1.05f32);
        Ray::new(self.origin, self.upper_left + u * self.horizontal + v * self.vertical - self.origin)
    }
}

pub struct Camera {
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Unit<Vec3>,
    pub vfov: f32,
    pub aspect: f32,
    position: Option<Vec2>,
    forward_enable: bool,
    backward_enable: bool,
    left_enable: bool,
    right_enable: bool,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32) -> Camera {
        Camera { lookfrom, lookat, vup: Unit::new_normalize(vup), vfov, aspect,
            position: None,
            forward_enable: false, backward_enable: false, left_enable: false, right_enable: false }
    }

    pub fn translate(&mut self, trans: &Vec3) {
        self.lookfrom += trans;
        self.lookat += trans;
    }

    pub fn forward(&self) -> Unit<Vec3> {
        Unit::new_normalize(self.lookat - self.lookfrom)
    }

    pub fn backward(&self) -> Unit<Vec3> {
        -self.forward()
    }

    pub fn left(&self) -> Unit<Vec3> {
        Unit::new_normalize(self.vup.cross(&self.forward()))
    }

    pub fn right(&self) -> Unit<Vec3> {
        Unit::new_normalize(-self.vup.cross(&self.forward()))
    }

    pub fn update(&mut self, dt: f32) {
        let mut dir = Vec3::zeros();

        if self.forward_enable {
            dir += self.forward().into_inner();
        }

        if self.backward_enable {
            dir += self.backward().into_inner();
        }

        if self.left_enable {
            dir += self.left().into_inner();
        }

        if self.right_enable {
            dir += self.right().into_inner();
        }

        if let Some(dir) = dir.try_normalize(0.1) {
            self.translate(&(dir * dt));
        }
    }

    pub fn process_input(&mut self, e: &Event) {
        match e {
            Event::WindowEvent { event: WindowEvent::CursorMoved { position, .. }, .. } =>
                self.process_mouse( Vec2::new(position.x as f32, position.y as f32)),
            Event::WindowEvent { event: WindowEvent::KeyboardInput { input, .. }, .. } => {
                let pressed = if input.state == ElementState::Pressed { true } else { false };

                if let Some(key) = input.virtual_keycode {
                    match key {
                        VirtualKeyCode::W => {
                            self.forward_enable = pressed;
                        },
                        VirtualKeyCode::S => {
                            self.backward_enable = pressed;
                        },
                        VirtualKeyCode::A => {
                            self.left_enable = pressed;
                        },
                        VirtualKeyCode::D => {
                            self.right_enable = pressed;
                        },
                        _ => {},
                    };
                }
            },
            _ => ()
        }
    }

    fn process_mouse(&mut self, pos: Vec2) {
        match self.position {
            None => self.position = Some(Vec2::new(400., 400.)), // !todo: wtf
            Some(prev_pos) => {
                let mouse_sense = 1. / 1000.;
                let diff = (pos - prev_pos) * mouse_sense;

                // !todo: dont change lookat
                let mut forward = self.forward().into_inner();

                let rotation = Rotation3::from_axis_angle(&self.right(), -diff.y);
                forward = rotation * forward;

                let rotation = Rotation3::from_axis_angle(&self.vup, -diff.x);
                forward = rotation * forward;

                self.lookat = self.lookfrom + forward;

//                self.position = Some(pos);
            },
        }
    }
}