use crate::vertex::Vertex;
use wgpu::{
    include_wgsl, BlendState, ColorTargetState, ColorWrites, Device, Face, FragmentState,
    FrontFace, MultisampleState, PipelineLayout, PipelineLayoutDescriptor, PolygonMode,
    PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, ShaderModule,
    SurfaceConfiguration, VertexState,
};

pub fn create_pipeline(
    device: &Device,
    config: &SurfaceConfiguration,
) -> (RenderPipeline, RenderPipeline) {
    let shader = device.create_shader_module(include_wgsl!("../resources/shader.wgsl"));

    let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let render_pipeline =
        create_pipeline_int(device, config, &render_pipeline_layout, &shader, "fs_main");

    let challenge_pipeline = create_pipeline_int(
        device,
        config,
        &render_pipeline_layout,
        &shader,
        "fs_challenge",
    );

    (render_pipeline, challenge_pipeline)
}

fn create_pipeline_int(
    device: &Device,
    config: &SurfaceConfiguration,
    pipeline_layout: &PipelineLayout,
    shader: &ShaderModule,
    fs_entry_point: &str,
) -> RenderPipeline {
    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("Render pipeline"),
        layout: Some(pipeline_layout),
        vertex: VertexState {
            module: shader,
            entry_point: "vs_main",
            buffers: &[Vertex::desc()],
        },
        fragment: Some(FragmentState {
            module: shader,
            entry_point: fs_entry_point,
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
