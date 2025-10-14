use crate::mesh::MeshConfig;
use crate::{BufferUsageHint, PrimitveType, ShaderDataType, VertexAttribute};
use gl::types::{GLsizei, GLsizeiptr, GLuint};

/// A simple mesh structure storing vertex data and OpenGL objects
pub struct Mesh {
    vao: Option<GLuint>, //Vertex Array Object
    vbo: Option<GLuint>, // Vertex Buffer Object
    ebo: Option<GLuint>, // Element Buffer Object
    instance_vbo: Option<GLuint>,
    vertex_count: i32,
    index_count: i32,
    instance_count: i32,
    primitive_type: Option<PrimitveType>,
}

impl Mesh {
    /// Create an empty mesh
    pub fn empty() -> Self {
        Self {
            vao: None,
            vbo: None,
            ebo: None,
            instance_vbo: None,
            vertex_count: 0,
            index_count: 0,
            instance_count: 0,
            primitive_type: None,
        }
    }

    pub fn new(mesh_config: MeshConfig) -> Mesh {
        // Extract required fields from config, handling potential 'None' values with panics
        // or return a sensible default/error if the context allows.
        // For simplicity and matching the spirit of the original 'unwrap()',
        // we'll use expect() here to clearly state what's missing if it panics.
        let vertices = mesh_config.vertices;
        let attributes = mesh_config.attributes;

        let indices = mesh_config.indices;
        let buffer_usage_hint = mesh_config.buffer_usage_hint;
        let primitive_type = mesh_config.primitive_type;

        // --- OpenGL Setup ---

        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;
        let mut ebo: GLuint = 0;

        let buffer_usage_hint_gl = match buffer_usage_hint {
            BufferUsageHint::StaticDraw => gl::STATIC_DRAW,
            BufferUsageHint::DynamicDraw => gl::DYNAMIC_DRAW,
            BufferUsageHint::StreamDraw => gl::STREAM_DRAW,
        };

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);

            // 1. Vertex buffer (VBO)
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                vertices.as_ptr() as *const _,
                buffer_usage_hint_gl,
            );

            // 2. Set vertex attributes
            let vertex_size_in_bytes = attributes
                .iter()
                .map(|a| a.stride as i32)
                .max()
                .unwrap_or(1) as GLsizei
                * std::mem::size_of::<f32>() as GLsizei;

            for attribute in &attributes {
                let offset = (attribute.offset * std::mem::size_of::<f32>()) as *const _;

                gl::EnableVertexAttribArray(attribute.location);
                gl::VertexAttribPointer(
                    attribute.location,
                    attribute.size,
                    gl::FLOAT,
                    gl::FALSE,
                    vertex_size_in_bytes, // Use the determined stride (max stride from attributes)
                    offset,
                );
            }

            // 3. Index buffer (EBO) (if provided)
            if let Some(ref idx) = indices {
                gl::GenBuffers(1, &mut ebo);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    (idx.len() * std::mem::size_of::<u32>()) as GLsizeiptr,
                    idx.as_ptr() as *const _,
                    buffer_usage_hint_gl,
                );
            }

            gl::BindVertexArray(0);
        }

        // Determine the stride for vertex count calculation
        let stride = attributes.iter().map(|a| a.stride).max().unwrap_or(1);

        Mesh {
            vao: Some(vao),
            vbo: Some(vbo),
            ebo: Some(ebo),
            instance_vbo: None,
            vertex_count: (vertices.len() as i32) / stride,
            index_count: indices.map_or(0, |i| i.len() as i32),
            instance_count: 0,
            primitive_type: Some(primitive_type),
        }
    }

    pub fn set_instance_attributes(
        &mut self,
        data: &[Vec<ShaderDataType>],
        starting_location: u32,
    ) {
        let mut flat_data: Vec<f32> = Vec::new();
        let mut attributes: Vec<VertexAttribute> = Vec::new();

        let mut current_location = starting_location;
        let mut offset = 0;

        // Build layout and flatten data
        for (_, value) in data[0].iter().enumerate() {
            match value {
                ShaderDataType::Int(_) => {
                    attributes.push(VertexAttribute::new(current_location, 1, 0, offset));
                    current_location += 1;
                    offset += 1;
                }
                ShaderDataType::Float(_) => {
                    attributes.push(VertexAttribute::new(current_location, 1, 0, offset));
                    current_location += 1;
                    offset += 1;
                }
                ShaderDataType::Vec2(_) => {
                    attributes.push(VertexAttribute::new(current_location, 2, 0, offset));
                    current_location += 1;
                    offset += 2;
                }
                ShaderDataType::Vec3(_) => {
                    attributes.push(VertexAttribute::new(current_location, 3, 0, offset));
                    current_location += 1;
                    offset += 3;
                }
                ShaderDataType::Vec4(_) => {
                    attributes.push(VertexAttribute::new(current_location, 4, 0, offset));
                    current_location += 1;
                    offset += 4;
                }
                ShaderDataType::Mat3(_) => {
                    for col in 0..3 {
                        attributes.push(VertexAttribute::new(
                            current_location,
                            3,
                            0,
                            offset + col * 3,
                        ));
                        current_location += 1;
                    }
                    offset += 9;
                }
                ShaderDataType::Mat4(_) => {
                    for col in 0..4 {
                        attributes.push(VertexAttribute::new(
                            current_location,
                            4,
                            0,
                            offset + col * 4,
                        ));
                        current_location += 1;
                    }
                    offset += 16;
                }
            }
        }

        let stride = offset; // total f32s per instance

        for instance in data {
            for value in instance {
                match value {
                    ShaderDataType::Int(i) => flat_data.push(*i as f32),
                    ShaderDataType::Float(f) => flat_data.push(*f),
                    ShaderDataType::Vec2(v) => {
                        flat_data.extend_from_slice(&[v.x, v.y]);
                    }
                    ShaderDataType::Vec3(v) => {
                        flat_data.extend_from_slice(&[v.x, v.y, v.z]);
                    }
                    ShaderDataType::Vec4(v) => {
                        flat_data.extend_from_slice(&[v.x, v.y, v.z, v.w]);
                    }
                    ShaderDataType::Mat3(m) => {
                        let mat: &[f32; 9] = m.as_ref(); // Mat3 column-major
                        flat_data.extend_from_slice(mat);
                    }
                    ShaderDataType::Mat4(m) => {
                        let mat: &[f32; 16] = m.as_ref(); // Mat4 column-major
                        flat_data.extend_from_slice(mat);
                    }
                }
            }
        }

        // Set stride for attributes
        for attr in &mut attributes {
            attr.stride = stride as i32;
        }

        self.set_instance_data(&flat_data, &attributes);
    }

    fn set_instance_data(&mut self, instance_data: &[f32], attributes: &[VertexAttribute]) {
        if cfg!(debug_assertions) {
            println!("Setting instance data.");
        }
        let vao = match self.vao {
            Some(id) => id,
            None => {
                eprintln!("Cannot set instance data without an initialized mesh.");
                return;
            }
        };

        unsafe {
            gl::BindVertexArray(vao);

            if let Some(instance_vbo) = self.instance_vbo {
                if cfg!(debug_assertions) {
                    println!("Updating old vertex buffer.");
                }
                // Update existing VBO
                gl::BindBuffer(gl::ARRAY_BUFFER, instance_vbo);
                gl::BufferSubData(
                    gl::ARRAY_BUFFER,
                    0,
                    (instance_data.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                    instance_data.as_ptr() as *const _,
                );
            } else {
                if cfg!(debug_assertions) {
                    println!("Creating new vertex buffer.\nIf you see this more than once, it's a memory leak!");
                }
                // Create new VBO
                let mut instance_vbo: GLuint = 0;
                gl::GenBuffers(1, &mut instance_vbo);
                gl::BindBuffer(gl::ARRAY_BUFFER, instance_vbo);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (instance_data.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                    instance_data.as_ptr() as *const _,
                    gl::STATIC_DRAW,
                );
                self.instance_vbo = Some(instance_vbo);
            }

            // Enable attributes and set divisor
            for attr in attributes {
                gl::EnableVertexAttribArray(attr.location);
                gl::VertexAttribPointer(
                    attr.location,
                    attr.size,
                    gl::FLOAT,
                    gl::FALSE,
                    (attr.stride * std::mem::size_of::<f32>() as i32) as GLsizei,
                    (attr.offset * std::mem::size_of::<f32>()) as *const _,
                );
                gl::VertexAttribDivisor(attr.location, 1);
            }

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        let stride = attributes.iter().map(|a| a.stride).max().unwrap_or(1);
        self.instance_count = (instance_data.len() as i32) / stride;
    }

    /// Draw the mesh if initialized
    pub fn draw(&self) {
        let vao = match self.vao {
            Some(vao) => vao,
            None => {
                eprintln!("Cannot draw this mesh without initializing first.");
                return;
            }
        };
        if cfg!(debug_assertions) {
            println!("Drawing mesh");
        }

        if let Some(vao) = self.vao {
            unsafe {
                let primitive_type = match self.primitive_type.as_ref().unwrap() {
                    PrimitveType::Point => gl::POINT,
                    PrimitveType::Line => gl::LINE,
                    PrimitveType::LineStrip => gl::LINE_STRIP,
                    PrimitveType::LineLoop => gl::LINE_LOOP,
                    PrimitveType::Triangle => gl::TRIANGLES,
                    PrimitveType::TriangleStrip => gl::TRIANGLE_STRIP,
                    PrimitveType::TriangleFan => gl::TRIANGLE_FAN,
                };

                gl::BindVertexArray(vao);

                if self.instance_count > 0 {
                    if self.index_count > 0 {
                        if cfg!(debug_assertions) {
                            println!("drawing instanced");
                        }
                        gl::DrawElementsInstanced(
                            primitive_type,
                            self.index_count,
                            gl::UNSIGNED_INT,
                            std::ptr::null(),
                            self.instance_count,
                        );
                        if cfg!(debug_assertions) {
                            println!("with set indices");
                        }
                    } else {
                        gl::DrawArraysInstanced(
                            primitive_type,
                            0,
                            self.vertex_count,
                            self.instance_count,
                        );
                        if cfg!(debug_assertions) {
                            println!("with auto indices");
                        }
                    }
                } else {
                    if cfg!(debug_assertions) {
                        println!("drawing non instanced");
                    }
                    if self.index_count > 0 {
                        gl::DrawElements(
                            primitive_type,
                            self.index_count,
                            gl::UNSIGNED_INT,
                            std::ptr::null(),
                        );
                        if cfg!(debug_assertions) {
                            println!("with set indices");
                        }
                    } else {
                        gl::DrawArrays(primitive_type, 0, self.vertex_count);
                        if cfg!(debug_assertions) {
                            println!("with auto indices");
                        }
                    }
                }

                gl::BindVertexArray(0);
            }
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        println!("Dropping mesh!");
        unsafe {
            // Delete the Vertex Array Object (VAO)
            if let Some(vao) = self.vao {
                gl::DeleteVertexArrays(1, &vao);
            }
            // Delete the Vertex Buffer Object (VBO)
            if let Some(vbo) = self.vbo {
                gl::DeleteBuffers(1, &vbo);
            }
            // Delete the Element Buffer Object (EBO)
            if let Some(ebo) = self.ebo {
                gl::DeleteBuffers(1, &ebo);
            }
            // Delete the Instance VBO
            if let Some(instance_vbo) = self.instance_vbo {
                gl::DeleteBuffers(1, &instance_vbo);
            }
        }
    }
}
