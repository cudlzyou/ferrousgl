use crate::{ShaderDataType, VertexAttribute};
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
        }
    }

    /// Initialize the mesh with vertex data
    pub fn new(&mut self, vertices: &[f32], attributes: &[VertexAttribute]) {
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            for attr in attributes {
                gl::EnableVertexAttribArray(attr.location);
                gl::VertexAttribPointer(
                    attr.location,
                    attr.size,
                    gl::FLOAT,
                    gl::FALSE,
                    (attr.stride * (std::mem::size_of::<f32>() as i32)) as GLsizei,
                    (attr.offset * std::mem::size_of::<f32>()) as *const _,
                );
            }

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        let stride = attributes.iter().map(|a| a.stride).max().unwrap_or(1);

        self.vao = Some(vao);
        self.vbo = Some(vbo);
        self.ebo = None;
        self.vertex_count = (vertices.len() as i32) / stride;
        self.index_count = 0;
    }

    /// Initialize the mesh with vertex data and indices
    pub fn new_with_indices(
        &mut self,
        vertices: &[f32],
        indices: &[u32],
        attributes: &[VertexAttribute],
    ) {
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;
        let mut ebo: GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            // Vertex buffer
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            for attr in attributes {
                gl::EnableVertexAttribArray(attr.location);
                gl::VertexAttribPointer(
                    attr.location,
                    attr.size,
                    gl::FLOAT,
                    gl::FALSE,
                    (attr.stride * (std::mem::size_of::<f32>() as i32)) as GLsizei,
                    (attr.offset * std::mem::size_of::<f32>()) as *const _,
                );
            }

            // Index buffer
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as GLsizeiptr,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindVertexArray(0);
        }

        let stride = attributes.iter().map(|a| a.stride).max().unwrap_or(1);

        self.vao = Some(vao);
        self.vbo = Some(vbo);
        self.ebo = None;
        self.vertex_count = (vertices.len() as i32) / stride;
        self.index_count = indices.len() as i32;
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
        let mut instance_vbo: GLuint = 0;

        unsafe {
            gl::GenBuffers(1, &mut instance_vbo);
            gl::BindVertexArray(self.vao.unwrap());
            gl::BindBuffer(gl::ARRAY_BUFFER, instance_vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (instance_data.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                instance_data.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            for attr in attributes {
                gl::EnableVertexAttribArray(attr.location);
                gl::VertexAttribPointer(
                    attr.location,
                    attr.size,
                    gl::FLOAT,
                    gl::FALSE,
                    (attr.stride * (std::mem::size_of::<f32>() as i32)) as GLsizei,
                    (attr.offset * std::mem::size_of::<f32>()) as *const _,
                );
                gl::VertexAttribDivisor(attr.location, 1); // <- Make it instanced
            }

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        let stride = attributes.iter().map(|a| a.stride).max().unwrap_or(1);

        self.instance_vbo = Some(instance_vbo);
        self.instance_count = (instance_data.len() as i32) / stride;
    }

    /// Draw the mesh if initialized
    pub fn draw(&self) {
        if let Some(vao) = self.vao {
            unsafe {
                gl::BindVertexArray(vao);

                if self.instance_count > 0 {
                    if self.index_count > 0 {
                        gl::DrawElementsInstanced(
                            gl::TRIANGLES,
                            self.index_count,
                            gl::UNSIGNED_INT,
                            std::ptr::null(),
                            self.instance_count,
                        );
                    } else {
                        gl::DrawArraysInstanced(
                            gl::TRIANGLES,
                            0,
                            self.vertex_count,
                            self.instance_count,
                        );
                    }
                } else {
                    if self.index_count > 0 {
                        gl::DrawElements(
                            gl::TRIANGLES,
                            self.index_count,
                            gl::UNSIGNED_INT,
                            std::ptr::null(),
                        );
                    } else {
                        gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
                    }
                }

                gl::BindVertexArray(0);
            }
        }
    }
}
