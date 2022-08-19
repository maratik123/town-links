use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, SquareMatrix};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Mat4x4Uniform {
    mat: [[f32; 4]; 4],
}

impl Mat4x4Uniform {
    #[inline]
    pub fn update_mat(&mut self, mat: impl Into<[[f32; 4]; 4]>) {
        *self = Self::setup(mat);
    }

    #[inline]
    fn setup(mat: impl Into<[[f32; 4]; 4]>) -> Self {
        Self { mat: mat.into() }
    }
}

impl Default for Mat4x4Uniform {
    #[inline]
    fn default() -> Self {
        Self::setup(Matrix4::identity())
    }
}
