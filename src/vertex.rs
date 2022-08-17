use bytemuck::{Pod, Zeroable};
use std::mem;
use wgpu::{vertex_attr_array, BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    const ATTRIBS: [VertexAttribute; 2] = vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    #[inline]
    pub const fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        tex_coords: [0.4131759, 1.0 - 0.99240386],
    },
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        tex_coords: [0.0048659444, 1.0 - 0.56958647],
    },
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        tex_coords: [0.28081453, 1.0 - 0.05060294],
    },
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        tex_coords: [0.85967, 1.0 - 0.1526709],
    },
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        tex_coords: [0.9414737, 1.0 - 0.7347359],
    },
    Vertex {
        position: [0.17732481, 0.46356988, 0.0],
        tex_coords: [0.6773248, 1.0 - 0.86356988],
    },
    Vertex {
        position: [0.10024225, -0.42836308, 0.0],
        tex_coords: [0.5702423, 1.0 - 0.10163692],
    },
];

pub const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];
pub const INDICES_CHALLENGE2: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4, 5, 0, 4, 2, 6, 3];
