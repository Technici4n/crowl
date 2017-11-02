extern crate cgmath;

use self::cgmath::prelude::*;
use self::cgmath::{Deg, Matrix4, Point3, Vector3, perspective};
use ::input::KeyboardState;

// TODO: Don't hardcode this
const MOVE_FORWARD: u32 = 17;
const MOVE_LEFT: u32 = 30;
const MOVE_BACKWARD: u32 = 31;
const MOVE_RIGHT: u32 = 32;
const MOVE_UP: u32 = 57;
const MOVE_DOWN: u32 = 42;
const CONTROL: u32 = 29;

const DEGS_PER_PIXEL: Deg<f32> = Deg(0.2);
const METERS_PER_SEC: f32 = 5.0;

pub struct Camera {
    position: Vector3<f32>,
    yaw: Deg<f32>,
    pitch: Deg<f32>,
    win_w: u32,
    win_h: u32,
}

impl Camera {
    pub fn new(win_w: u32, win_h: u32) -> Camera {
        Camera {
            position: Vector3::from([0.0, 0.0, 4.0]),
            yaw: Deg(0.0),
            pitch: Deg(0.0),
            win_w,
            win_h,
        }
    }

    // TODO: Allow mouse inverting
    pub fn update_cursor(&mut self, dx: f32, dy: f32) {
        self.yaw += -DEGS_PER_PIXEL * (dx as f32);
        self.pitch += -DEGS_PER_PIXEL * (dy as f32);

        // Ensure the pitch stays within [-90; 90]
        if self.pitch < Deg(-90.0) {
            self.pitch = Deg(-90.0);
        }
        if self.pitch > Deg(90.0) {
            self.pitch = Deg(90.0);
        }
    }

    fn get_mv_direction(&self, angle: Deg<f32>) -> Vector3<f32> {
        let yaw = self.yaw + angle;
        Vector3 {
            x: -yaw.sin(),
            y: 0.0,
            z: -yaw.cos(),
        }
    }

    pub fn tick(&mut self, dt: f32, keys: &KeyboardState) {
        let mut speedup = 1.0f32;
        if keys.is_key_pressed(CONTROL)
        { speedup = 15.0; }
        if keys.is_key_pressed(MOVE_FORWARD)
        { self.position += speedup * self.get_mv_direction(Deg(0.0)).normalize() * (METERS_PER_SEC * dt); }
        if keys.is_key_pressed(MOVE_LEFT)
        { self.position += speedup * self.get_mv_direction(Deg(90.0)).normalize() * (METERS_PER_SEC * dt); }
        if keys.is_key_pressed(MOVE_BACKWARD)
        { self.position += speedup * self.get_mv_direction(Deg(180.0)).normalize() * (METERS_PER_SEC * dt); }
        if keys.is_key_pressed(MOVE_RIGHT)
        { self.position += speedup * self.get_mv_direction(Deg(270.0)).normalize() * (METERS_PER_SEC * dt); }
        if keys.is_key_pressed(MOVE_UP)
        { self.position.y += speedup * METERS_PER_SEC * dt; }
        if keys.is_key_pressed(MOVE_DOWN)
        { self.position.y -= speedup * METERS_PER_SEC * dt; }
    }

    fn get_aspect_ratio(&self) -> f32 {
        self.win_w as f32 / self.win_h as f32
    }

    pub fn resize_window(&mut self, win_w: u32, win_h: u32) {
        self.win_w = win_w;
        self.win_h = win_h;
    }

    pub fn get_view_projection(&self) -> Matrix4<f32> {
        let cam_dir: Vector3<f32> = Vector3 {
            x: -self.pitch.cos() * self.yaw.sin(),
            y:  self.pitch.sin(),
            z: -self.pitch.cos() * self.yaw.cos(),
        };
        let right = Vector3 {
            x: -(self.yaw - Deg(90.0)).sin(),
            y:  0.0,
            z: -(self.yaw - Deg(90.0)).cos(),
        };
        let up = right.cross(cam_dir);

        let proj = perspective(Deg(45.0), self.get_aspect_ratio(), 0.1, 400.0);
        let view = Matrix4::look_at(
            Point3::from_vec(self.position),
            Point3::from_vec(self.position + cam_dir),
            up);

        proj * view
    }

    pub fn get_pos(&self) -> (f32, f32, f32) {
        (self.position[0], self.position[1], self.position[2])
    }
}