use crate::err::Error;
use image::{load_from_memory_with_format, DynamicImage, GenericImageView, ImageFormat};
use std::num::NonZeroU32;
use wgpu::{
    AddressMode, Device, Extent3d, FilterMode, ImageCopyTexture, ImageDataLayout, Origin3d, Queue,
    Sampler, SamplerDescriptor, Texture, TextureAspect, TextureDescriptor, TextureDimension,
    TextureFormat, TextureUsages, TextureView, TextureViewDescriptor,
};

pub struct TextureState {
    pub texture: Texture,
    pub view: TextureView,
    pub sampler: Sampler,
}

impl TextureState {
    pub fn from_bytes(
        device: &Device,
        queue: &Queue,
        bytes: &[u8],
        format: ImageFormat,
        label: &str,
    ) -> Result<Self, Error> {
        let img = load_from_memory_with_format(bytes, format)?;
        Self::from_image(device, queue, &img, Some(label))
    }

    pub fn from_image(
        device: &Device,
        queue: &Queue,
        img: &DynamicImage,
        label: Option<&str>,
    ) -> Result<Self, Error> {
        let rgba = img.to_rgba8();
        let (dimension_x, dimension_y) = img.dimensions();

        let size = Extent3d {
            width: dimension_x,
            height: dimension_y,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
        });

        queue.write_texture(
            ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            &rgba,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * dimension_x),
                rows_per_image: NonZeroU32::new(dimension_y),
            },
            size,
        );

        let view = texture.create_view(&TextureViewDescriptor::default());
        let sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self {
            texture,
            view,
            sampler,
        })
    }
}
