use bytemuck::{Pod, Zeroable};
use std::mem;
use wgpu::{vertex_attr_array, BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [0.17732481, 0.46356988, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [0.10024225, -0.42836308, 0.0],
        color: [0.5, 0.0, 0.5],
    },
];

pub const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];
pub const INDICES_CHALLENGE2: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4, 5, 0, 4, 2, 6, 3];

impl Vertex {
    const ATTRIBS: [VertexAttribute; 2] = vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    #[inline]
    pub const fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
