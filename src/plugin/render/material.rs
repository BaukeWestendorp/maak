use std::path::Path;

use bevy_ecs::resource::Resource;

use crate::plugin::render::RenderState;

#[derive(Debug, Resource)]
pub struct Material {
    pub pipeline: wgpu::RenderPipeline,
}

impl Material {
    pub fn new(render_state: &RenderState, shader_path: impl AsRef<Path>) -> Self {
        let shader_source =
            std::fs::read_to_string(shader_path).expect("Failed to read WGSL shader file");

        let shader = render_state
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Material Shader Module"),
                source: wgpu::ShaderSource::Wgsl(shader_source.into()),
            });

        let pipeline_layout =
            render_state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Material Pipeline Layout"),
                    bind_group_layouts: &[Some(&render_state.camera_layout)],
                    immediate_size: 0,
                });

        let pipeline =
            render_state
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Material Render Pipeline"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: Some("vs_main"),
                        buffers: &[Some(wgpu::VertexBufferLayout {
                            array_stride: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &[wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x3,
                            }],
                        })],
                        compilation_options: Default::default(),
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: Some("fs_main"),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: render_state.surface_format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                        compilation_options: Default::default(),
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        polygon_mode: wgpu::PolygonMode::Fill,
                        unclipped_depth: false,
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState::default(),
                    multiview_mask: None,
                    cache: None,
                });

        Self { pipeline }
    }
}
