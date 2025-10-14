use glam::{ Mat3, Mat4, Vec2, Vec3, Vec4 };

pub enum ShaderDataType {
    Int(i32),
    Float(f32),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
    Mat3(Mat3),
    Mat4(Mat4),
}

pub enum BufferUsageHint {
    StaticDraw,
    DynamicDraw,
    StreamDraw,
}

pub enum PrimitveType {
    Point,
    Line,
    LineStrip,
    LineLoop,
    Triangle,
    TriangleStrip,
    TriangleFan,
}

pub struct VertexAttribute {
    pub location: u32,
    pub size: i32,
    pub stride: i32,
    pub offset: usize,
}

impl VertexAttribute {
    pub fn new(location: u32, size: i32, stride: i32, offset: usize) -> Self {
        Self {
            location,
            size,
            stride,
            offset,
        }
    }
}