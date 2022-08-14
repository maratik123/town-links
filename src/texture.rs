use wgpu::{Sampler, Texture, TextureView};

pub struct TextureState {
    texture: Texture,
    view: TextureView,
    sampler: Sampler,
}
