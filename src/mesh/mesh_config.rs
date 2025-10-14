use crate::{BufferUsageHint, PrimitveType, VertexAttribute};
use crate::BufferUsageHint::{DynamicDraw, StaticDraw};

pub struct MeshConfig {
    pub vertices: Vec<f32>,
    pub indices: Option<Vec<u32>>,
    pub attributes: Vec<VertexAttribute>,
    pub buffer_usage_hint: BufferUsageHint,
    pub primitive_type: PrimitveType,
}

impl Default for MeshConfig {
    fn default() -> Self {
        Self {
            vertices: vec![],
            indices: None,
            attributes: vec![],
            buffer_usage_hint: StaticDraw,
            primitive_type: PrimitveType::Triangle,
        }
    }
}