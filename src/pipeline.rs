use crate::vertex::Vertex;
use wgpu::{
    include_wgsl, BindGroupLayout, BlendState, ColorTargetState, ColorWrites, Device, Face,
    FragmentState, FrontFace, MultisampleState, PipelineLayout, PipelineLayoutDescriptor,
    PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor,
    ShaderModule, SurfaceConfiguration, VertexBufferLayout, VertexState,
};

pub fn create_pipeline<'a>(
    device: &Device,
    config: &SurfaceConfiguration,
    bind_group_layouts: &'a [&'a BindGroupLayout],
) -> (RenderPipeline, RenderPipeline, RenderPipeline) {
    let shader = device.create_shader_module(include_wgsl!("../resources/shader.wgsl"));
    let challenge_shader =
        device.create_shader_module(include_wgsl!("../resources/challenge.wgsl"));
    let challenge4_shader =
        device.create_shader_module(include_wgsl!("../resources/challenge4.wgsl"));

    let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts,
        push_constant_ranges: &[],
    });

    let challenge_render_pipeline_layout =
        device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Challenge Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

    let challenge4_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Challenge4 Pipeline Layout"),
        bind_group_layouts,
        push_constant_ranges: &[],
    });

    let render_pipeline = create_pipeline_int(
        device,
        config,
        &render_pipeline_layout,
        &shader,
        &[Vertex::desc()],
    );

    let challenge_pipeline = create_pipeline_int(
        device,
        config,
        &challenge_render_pipeline_layout,
        &challenge_shader,
        &[],
    );

    let challenge4_pipeline = create_pipeline_int(
        device,
        config,
        &challenge4_pipeline_layout,
        &shader,
        &[Vertex::desc()],
    );

    (render_pipeline, challenge_pipeline, challenge4_pipeline)
}

fn create_pipeline_int<'a>(
    device: &Device,
    config: &SurfaceConfiguration,
    pipeline_layout: &PipelineLayout,
    shader: &ShaderModule,
    buffers: &'a [VertexBufferLayout<'a>],
) -> RenderPipeline {
    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("Render pipeline"),
        layout: Some(pipeline_layout),
        vertex: VertexState {
            module: shader,
            entry_point: "vs_main",
            buffers,
        },
        fragment: Some(FragmentState {
            module: shader,
            entry_point: "fs_main",
            targets: &[Some(ColorTargetState {
                format: config.format,
                blend: Some(BlendState::REPLACE),
                write_mask: ColorWrites::ALL,
            })],
        }),
        primitive: PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: Some(Face::Back),
            polygon_mode: PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    })
}
