use std::{ collections::HashMap, ffi::CString, fs, path::Path, ptr };
use gl;
use crate::ShaderDataType;

pub struct Shader {
    shader_program_id: u32,
    uniform_cache: HashMap<String, i32>,
}

impl Shader {
    /// Create an empty shader object. Use it to declare it outside somewhere and initialize after The gl context was loaded.
    pub fn empty() -> Shader {
        Shader {
            shader_program_id: 0, // 0 means "no OpenGL program yet"
            uniform_cache: HashMap::new(),
        }
    }

    pub fn new_from_file(vertex_path: &Path, fragment_path: &Path) -> Result<Shader, String> {
        let vertex_source = fs
            ::read_to_string(&vertex_path)
            .map_err(|e| format!("Failed to read vertex shader: {}", e))?;

        let fragment_source = fs
            ::read_to_string(&fragment_path)
            .map_err(|e| format!("Failed to read fragment shader: {}", e))?;

        Ok(
            Shader::new_from_sources(
                &[
                    (gl::VERTEX_SHADER, &vertex_source),
                    (gl::FRAGMENT_SHADER, &fragment_source),
                ]
            )?
        )
    }

    pub fn new_from_file_geometry(
        vertex_path: &Path,
        geometry_path: &Path,
        fragment_path: &Path
    ) -> Result<Shader, String> {
        let vertex_source = fs
            ::read_to_string(vertex_path)
            .map_err(|e| format!("Failed to read vertex shader: {}", e))?;

        let geometry_source = fs
            ::read_to_string(geometry_path)
            .map_err(|e| format!("Failed to read geometry shader: {}", e))?;

        let fragment_source = fs
            ::read_to_string(fragment_path)
            .map_err(|e| format!("Failed to read fragment shader: {}", e))?;

        Ok(
            Shader::new_from_sources(
                &[
                    (gl::VERTEX_SHADER, &vertex_source),
                    (gl::GEOMETRY_SHADER, &geometry_source),
                    (gl::FRAGMENT_SHADER, &fragment_source),
                ]
            )?
        )
    }

    // Compute shader constructor
    pub fn new_from_file_compute(compute_path: &Path) -> Result<Shader, String> {
        let compute_source = fs
            ::read_to_string(compute_path)
            .map_err(|e| format!("Failed to read compute shader: {}", e))?;

        Ok(Shader::new_from_sources(&[(gl::COMPUTE_SHADER, &compute_source)]).unwrap())
    }

    pub fn new_from_sources(shaders: &[(u32, &str)]) -> Result<Shader, String> {
        let program_id = unsafe { gl::CreateProgram() };
        let mut shader_ids = Vec::new();

        for &(shader_type, source) in shaders {
            let shader_id = unsafe { gl::CreateShader(shader_type) };
            // Compile the shader
            Self::compile(shader_id, source)?;
            // Attach to the program
            unsafe {
                gl::AttachShader(program_id, shader_id);
            }
            shader_ids.push(shader_id);
        }

        let shader = Shader {
            shader_program_id: program_id,
            uniform_cache: HashMap::new(),
        };

        // Link the program
        shader.link()?;

        // Clean up individual shaders after linking
        for &id in &shader_ids {
            unsafe {
                gl::DeleteShader(id);
            }
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
                let mut buffer: Vec<u8> = Vec::with_capacity((len as usize) + 1);

                gl::GetShaderInfoLog(
                    shader_id,
                    len,
                    ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut i8
                );

                buffer.set_len(len as usize);
                let log = String::from_utf8_lossy(&buffer).into_owned();
                return Err(log);
            }

            Ok(())
        }
    }

    fn link(&self) -> Result<(), String> {
        unsafe {
            gl::LinkProgram(self.shader_program_id);

            let mut success = 0;
            gl::GetProgramiv(self.shader_program_id, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut len = 0;
                gl::GetProgramiv(self.shader_program_id, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer: Vec<u8> = Vec::with_capacity((len as usize) + 1);

                gl::GetProgramInfoLog(
                    self.shader_program_id,
                    len,
                    ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut i8
                );

                buffer.set_len(len as usize);
                let log = String::from_utf8_lossy(&buffer).into_owned();
                return Err(log);
            }
        }
        Ok(())
    }

    pub fn bind(&self) {
        if self.shader_program_id != 0 {
            unsafe {
                gl::UseProgram(self.shader_program_id);
            }
        } else {
            panic!("Shader is not initialized!")
        }
    }

    fn get_uniform_location(&mut self, name: &str) -> i32 {
        if let Some(&loc) = self.uniform_cache.get(name) {
            return loc;
        }

        let c_name = CString::new(name).expect("Uniform name contained null byte");
        let location = unsafe { gl::GetUniformLocation(self.shader_program_id, c_name.as_ptr()) };

        // Cache the location (even if -1, so we don't keep asking GL)
        self.uniform_cache.insert(name.to_string(), location);
        location
    }

    pub fn set_uniform(&mut self, name: &str, value: ShaderDataType) {
        let location = self.get_uniform_location(name);
        if location == -1 {
            // Uniform not found in shader, silently ignore or log
            return;
        }

        unsafe {
            match value {
                ShaderDataType::Int(v) => gl::Uniform1i(location, v),
                ShaderDataType::Float(v) => gl::Uniform1f(location, v),
                ShaderDataType::Vec2(v) => gl::Uniform2f(location, v.x, v.y),
                ShaderDataType::Vec3(v) => gl::Uniform3f(location, v.x, v.y, v.z),
                ShaderDataType::Vec4(v) => gl::Uniform4f(location, v.x, v.y, v.z, v.w),
                ShaderDataType::Mat3(m) => {
                    gl::UniformMatrix3fv(location, 1, gl::FALSE, m.to_cols_array().as_ptr())
                }
                ShaderDataType::Mat4(m) => {
                    gl::UniformMatrix4fv(location, 1, gl::FALSE, m.to_cols_array().as_ptr())
                }
            }
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        println!("Dropping shader: {}", self.shader_program_id);
        unsafe {
            if self.shader_program_id != 0 {
                gl::DeleteProgram(self.shader_program_id);
            }
        }
    }
}
