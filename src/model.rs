use crate::mat4x4_uniform::Mat4x4Uniform;
use bytemuck::{Pod, Zeroable};
use cgmath::{Deg, Matrix4};

pub struct Model {
    rotate_over_x: f32,
}

impl Model {
    #[inline]
    pub fn build_model_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_angle_x(Deg(self.rotate_over_x))
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable, Default)]
pub struct ModelUniform(Mat4x4Uniform);

impl ModelUniform {
    #[inline]
    pub fn update_model(&mut self, model: &Model) {
        let Self(mat) = self;
        mat.update_mat(model.build_model_matrix());
    }
}
