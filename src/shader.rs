use std::{ffi::CString, fs, path::Path, ptr};

use gl;

pub struct Shader {
    shader_program_id: u32,
}

impl Shader {
    pub fn new_from_file(vertex_path: &Path, fragment_path: &Path) -> Result<Shader, String> {
        let vertex_source = fs::read_to_string(&vertex_path)
            .map_err(|e| format!("Failed to read vertex shader: {}", e))?;

        let fragment_source = fs::read_to_string(&fragment_path)
            .map_err(|e| format!("Failed to read fragment shader: {}", e))?;

        Ok(Shader::new_from_sources(&[(gl::VERTEX_SHADER, &vertex_source), (gl::FRAGMENT_SHADER, &fragment_source)]).unwrap())
    }

    pub fn new_from_file_geometry(vertex_path: &Path, geometry_path: &Path, fragment_path: &Path) -> Result<Shader, String> {
        let vertex_source = fs::read_to_string(vertex_path)
            .map_err(|e| format!("Failed to read vertex shader: {}", e))?;

        let geometry_source = fs::read_to_string(geometry_path)
            .map_err(|e| format!("Failed to read geometry shader: {}", e))?;

        let fragment_source = fs::read_to_string(fragment_path)
            .map_err(|e| format!("Failed to read fragment shader: {}", e))?;

        Ok(Shader::new_from_sources(&[
            (gl::VERTEX_SHADER, &vertex_source),
            (gl::GEOMETRY_SHADER, &geometry_source),
            (gl::FRAGMENT_SHADER, &fragment_source),
        ]).unwrap())
    }

    // Compute shader constructor
    pub fn new_from_file_compute(compute_path: &Path) -> Result<Shader, String> {
        let compute_source = fs::read_to_string(compute_path)
            .map_err(|e| format!("Failed to read compute shader: {}", e))?;

        Ok(Shader::new_from_sources(&[
            (gl::COMPUTE_SHADER, &compute_source),
        ]).unwrap())
    }

    pub fn new_from_sources(shaders: &[(u32, &str)]) -> Result<Shader, String> {
        let program_id = unsafe { gl::CreateProgram() };
        let mut shader_ids = Vec::new();

        for &(shader_type, source) in shaders {
            let shader_id = unsafe { gl::CreateShader(shader_type) };
            // Compile the shader
            Self::compile(shader_id, source)?;
            // Attach to the program
            unsafe { gl::AttachShader(program_id, shader_id) };
            shader_ids.push(shader_id);
        }

        let shader = Shader {
            shader_program_id: program_id,
        };

        // Link the program
        shader.link()?;

        // Clean up individual shaders after linking
        for &id in &shader_ids {
            unsafe { gl::DeleteShader(id) };
        }

        Ok(shader)
    }

    pub fn compile(shader_id: u32, source: &str) -> Result<(), String> {
        unsafe {
            let source_as_c_string = CString::new(source).unwrap();
            gl::ShaderSource(shader_id, 1, &source_as_c_string.as_ptr(), ptr::null());

            gl::CompileShader(shader_id);

            let mut success = 0;
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut len = 0;
                gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);

                gl::GetShaderInfoLog(
                    shader_id,
                    len,
                    ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut i8,
                );

                buffer.set_len(len as usize);
                let log = String::from_utf8_lossy(&buffer).into_owned();
                return Err(log);
            }

            Ok(())
        }
    }

    pub fn link(&self) -> Result<(), String> {
        unsafe {
            gl::LinkProgram(self.shader_program_id);

            let mut success = 0;
            gl::GetProgramiv(self.shader_program_id, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut len = 0;
                gl::GetProgramiv(self.shader_program_id, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);

                gl::GetProgramInfoLog(
                    self.shader_program_id,
                    len,
                    ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut i8,
                );

                buffer.set_len(len as usize);
                let log = String::from_utf8_lossy(&buffer).into_owned();
                return Err(log);
            }
        }
        Ok(())
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.shader_program_id);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        println!("Dropping shader: {}", self.shader_program_id);
        unsafe {
            gl::DeleteProgram(self.shader_program_id);
        }
    }
}
