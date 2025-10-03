use gl::types::{GLsizei, GLsizeiptr, GLuint};

/// A simple mesh structure storing vertex data and OpenGL objects
pub struct Mesh {
    vao: Option<GLuint>,
    vbo: Option<GLuint>,
    ebo: Option<GLuint>,
    vertex_count: i32,
    index_count: i32,
}

impl Mesh {
    /// Create an empty mesh
    pub fn empty() -> Self {
        Self {
            vao: None,
            vbo: None,
            ebo: None,
            vertex_count: 0,
            index_count: 0,
        }
    }

    /// Initialize the mesh with vertex data
    pub fn new(&mut self, vertices: &[f32]) {
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

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * std::mem::size_of::<f32>() as GLsizei,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        self.vao = Some(vao);
        self.vbo = Some(vbo);
        self.vertex_count = (vertices.len() / 3) as i32;
        self.index_count = 0;
    }

    /// Initialize the mesh with vertex data and indices
    pub fn new_with_indices(&mut self, vertices: &[f32], indices: &[u32]) {
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

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * std::mem::size_of::<f32>() as GLsizei,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            // Element buffer
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as GLsizeiptr,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindVertexArray(0);
        }

        self.vao = Some(vao);
        self.vbo = Some(vbo);
        self.ebo = Some(ebo);
        self.vertex_count = (vertices.len() / 3) as i32;
        self.index_count = indices.len() as i32;
    }

    /// Draw the mesh if initialized
    pub fn draw(&self) {
        if let Some(vao) = self.vao {
            unsafe {
                gl::BindVertexArray(vao);
                if self.index_count > 0 {
                    gl::DrawElements(gl::TRIANGLES, self.index_count, gl::UNSIGNED_INT, std::ptr::null());
                } else {
                    gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
                }
                gl::BindVertexArray(0);
            }
        }
    }
}