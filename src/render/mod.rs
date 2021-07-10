use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        pipeline::{
            BlendFactor, BlendOperation, BlendState, ColorTargetState, ColorWrite, CompareFunction,
            DepthBiasState, DepthStencilState, PipelineDescriptor,
            StencilFaceState, StencilState,
        },
        shader::{ShaderStage, ShaderStages},
        texture::TextureFormat,
    },
};

macro_rules! create_chunk_pipeline {
    ($pipeline_handle: ident, $pipeline_id: expr, $function: ident, $vert_file: expr, $frag_file: expr) => {
        /// The constant render pipeline for a chunk.
        pub(crate) const $pipeline_handle: HandleUntyped =
            HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, $pipeline_id);

        /// Builds the chunk render pipeline.
        fn $function(shaders: &mut Assets<Shader>) -> PipelineDescriptor {
            PipelineDescriptor {
                color_target_states: vec![ColorTargetState {
                    format: TextureFormat::default(),
                    color_blend: BlendState {
                        src_factor: BlendFactor::SrcAlpha,
                        dst_factor: BlendFactor::OneMinusSrcAlpha,
                        operation: BlendOperation::Add,
                    },
                    alpha_blend: BlendState {
                        src_factor: BlendFactor::One,
                        dst_factor: BlendFactor::One,
                        operation: BlendOperation::Add,
                    },
                    write_mask: ColorWrite::ALL,
                }],
                depth_stencil: Some(DepthStencilState {
                    format: TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: CompareFunction::LessEqual,
                    stencil: StencilState {
                        front: StencilFaceState::IGNORE,
                        back: StencilFaceState::IGNORE,
                        read_mask: 0,
                        write_mask: 0,
                    },
                    bias: DepthBiasState {
                        constant: 0,
                        slope_scale: 0.0,
                        clamp: 0.0,
                    },
                    clamp_depth: false,
                }),
                ..PipelineDescriptor::new(ShaderStages {
                    vertex: shaders.add(Shader::from_glsl(
                        ShaderStage::Vertex,
                        include_str!($vert_file),
                    )),
                    fragment: Some(shaders.add(Shader::from_glsl(
                        ShaderStage::Fragment,
                        include_str!($frag_file),
                    ))),
                })
            }
        }
    };
}

create_chunk_pipeline!(
    SQUARE_PIPELINE,
    8094008129742002341,
    create_square_pipeline,
    "square-tilemap.vert",
    "square-tilemap.frag"
);

pub(crate) fn add_tile_map_graph(world: &mut World) {
    world.resource_scope(|world, mut pipelines: Mut<Assets<PipelineDescriptor>>| {
        world.resource_scope(|_, mut shaders: Mut<Assets<Shader>>| {
            pipelines.set_untracked(SQUARE_PIPELINE, create_square_pipeline(&mut shaders));
        });
    });
}