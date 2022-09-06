use vecmath::Matrix4;
use crate::math::linear_algebra::{IDENTITY_MAT4, orthographic, perspective};

#[derive(Debug, Clone)]
pub struct Camera {
    fov: f32,
    far: f32,
    near: f32,
    aspect_ratio: f32,

    view: Matrix4<f32>,
    last_perspective: Matrix4<f32>,
    last_orthographic: Matrix4<f32>,
}

impl Camera {
    pub fn new(fov: f32, far: f32, near: f32, draw_size: [i32; 2]) -> Camera {
        let aspect_ratio = draw_size[0] as f32 / draw_size[1] as f32;
        Self {
            fov,
            far,
            near,
            aspect_ratio,
            view: IDENTITY_MAT4,
            last_perspective: perspective(fov, aspect_ratio, near, far),
            last_orthographic: orthographic(draw_size[0], draw_size[1], -1f32, 1f32),
        }
    }

    pub fn update_aspect_ratio(&mut self, draw_size: [i32; 2]) {
        let aspect_ratio = draw_size[0] as f32 / draw_size[1] as f32;
        self.aspect_ratio = aspect_ratio;
        self.update_perspective();
        self.update_orthographic(draw_size);
    }

    pub fn update_perspective(&mut self) {
        self.last_perspective = perspective(self.fov, self.aspect_ratio, self.near, self.far);
    }

    pub fn update_orthographic(&mut self, draw_size: [i32; 2]) {
        self.last_orthographic = orthographic(draw_size[0], draw_size[1], -1f32, 1f32);
    }

    pub fn view(&self) -> &Matrix4<f32> {
        &self.view
    }

    pub fn fov(&self) -> f32 {
        self.fov
    }
    pub fn far(&self) -> f32 {
        self.far
    }
    pub fn near(&self) -> f32 {
        self.near
    }
    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }
    pub fn last_perspective(&self) -> &Matrix4<f32> {
        &self.last_perspective
    }
    pub fn last_orthographic(&self) -> &Matrix4<f32> {
        &self.last_orthographic
    }
}