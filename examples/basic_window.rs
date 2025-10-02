use std::path::Path;
use ferrousgl2::*;
use glam::{Vec3, Mat4};

fn main() {
    let window_config = WindowConfig {
        decorated: true,
        framerate: Some(60),
        hide_cursor: false,
        fullscreen: false,
        size: (300, 300),
        title: "eee".to_string(),
        translucent: false,
        vsync: false,
    };
    let gl_config = GlConfig {
        version_major: 1,
        version_minor: 1,
        ..Default::default()
    };

    let mut window = Window::new(window_config, gl_config);

    let mut shader = Shader::empty();
    let mut mesh = Mesh::empty();

    window.set_update_callback(move |handle| {
        if handle.just_initialized() {
            // Load enhanced shaders
            shader = Shader::new_from_file(
                Path::new("./examples/assets/shaders/color.vert"),
                Path::new("./examples/assets/shaders/color.frag"),
            )
            .unwrap();

            // Triangle vertices
            mesh.new(&[
                 0.0,  0.5, 0.0,  // top
                -0.5, -0.5, 0.0,  // bottom left
                 0.5, -0.5, 0.0,  // bottom right
            ]);
        } else {
            let frame = handle.get_frames_count() as f32;
            let t = frame * 0.02;
            let frame_time = handle.get_frame_time();

            // Rainbow color animation
            let color = Vec3::new(
                (t).sin() * 0.5 + 0.5,
                (t + 2.0).sin() * 0.5 + 0.5,
                (t + 4.0).sin() * 0.5 + 0.5,
            );

            // Background gradient based on time
            let bg_color_top = Vec3::new((t * 0.5).sin() * 0.5 + 0.5, 0.1, 0.3);
            let bg_color_bottom = Vec3::new(0.05, (t * 0.3).sin() * 0.5 + 0.5, 0.1);

            unsafe {
                gl::ClearColor(
                    (bg_color_top.x + bg_color_bottom.x) * 0.5,
                    (bg_color_top.y + bg_color_bottom.y) * 0.5,
                    (bg_color_top.z + bg_color_bottom.z) * 0.5,
                    1.0,
                );
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            // Dynamic transform
            let rotation = Mat4::from_rotation_z(t);
            let scale_factor = t.sin() * 0.25 + 1.0;
            let scale = Mat4::from_scale(Vec3::splat(scale_factor));
            let transform = rotation * scale;

            // Vertex wobble for extra visual flair
            let wobble = Mat4::from_translation(Vec3::new(0.0, (t * 3.0).sin() * 0.1, 0.0));

            handle.set_title(&format!("Spinning Rainbow Triangle! Frame Time {}", 1_000_000.0 / frame_time as f32));

            shader.bind();
            shader.set_uniform("colorOne", UniformValue::Vec3(color));
            shader.set_uniform("transform", UniformValue::Mat4(transform * wobble));

            mesh.draw();
        }
    });

    window.start_event_loop();
}
