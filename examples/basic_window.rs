use std::path::Path;

use ferrousgl2::*;
use glam::Vec3;

fn main() {
    let window_config = WindowConfig::default();
    let gl_config = GlConfig::default();

    // GL context is created and loaded immediately
    let mut window = Window::new(window_config, gl_config);

    let mut frames = 0;

    // The callback now receives a WindowHandle reference
    window.set_update_callback(move |handle| {
        if frames == 0 {
            let mut shader = Shader::new_from_file(Path::new("./examples/assets/shaders/color.vert"), Path::new("./examples/assets/shaders/color.frag")).unwrap();
        }
        frames += 1;

        // Access window properties
        if let Some(size) = handle.get_size() {
            println!("Window size: {}x{}", size.0, size.1);
        }

        // Can request redraws, change title, etc.s
        handle.set_title("Frame rendered!");

        //shader.bind();
        //shader.set_uniform("colorOne", UniformValue::Vec3(Vec3::new(0.3, 1.0, 0.5)));

        // Your GL rendering code here
        unsafe {
            gl::ClearColor(0.1, 0.2, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    });

    window.start_event_loop();
}