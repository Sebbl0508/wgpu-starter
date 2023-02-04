use crate::utils::OPENGL_TO_WGPU_MATRIX;
use cgmath::prelude::*;
use cgmath::Rad;
use cgmath::Vector3;
use cgmath::{perspective, Matrix4, Point3};

#[derive(Debug, Clone)]
pub struct Camera3D {
    position: Point3<f32>,
    // Yaw(0) Pitch(0) will result in X(1) Y(0) Z(0)
    yaw: Rad<f32>,
    pitch: Rad<f32>,
}

#[derive(Debug, Clone)]
pub struct Projection {
    aspect: f32,
    fovy: Rad<f32>,
    znear: f32,
    zfar: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_position: [f32; 4],
    view_proj: [[f32; 4]; 4],
}

impl Camera3D {
    pub fn new(
        position: impl Into<Point3<f32>>,
        yaw: impl Into<Rad<f32>>,
        pitch: impl Into<Rad<f32>>,
    ) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        }
    }

    /// Converts the yaw & pitch values to a direction vector
    pub fn view_dir(&self) -> Vector3<f32> {
        let xz_len = self.pitch.cos();
        let x = xz_len * self.yaw.cos();
        let y = self.pitch.sin();
        let z = xz_len * (-self.yaw).sin();

        Vector3::new(x, y, z)
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        Matrix4::look_to_rh(
            self.position,
            Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            Vector3::unit_y(),
        )
    }
}

impl Projection {
    pub fn new(width: u32, height: u32, fovy: impl Into<Rad<f32>>, znear: f32, zfar: f32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            znear,
            zfar,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX * perspective(self.fovy, self.aspect, self.znear, self.zfar)
    }
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_position: [0.0; 4],
            view_proj: Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera3D, projection: &Projection) {
        self.view_position = camera.position.to_homogeneous().into();
        self.view_proj = (projection.calc_matrix() * camera.calc_matrix()).into();
    }
}
