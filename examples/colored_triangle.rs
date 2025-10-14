use ferrousgl2::*;
use glam::Mat4;
use std::path::Path;

fn main() {
    // Initialize the default window configurations and OpenGL graphics
    let window_config = WindowConfig {
        title: "Colored Triangle Example".to_string(),
        ..Default::default()
    };
    let gl_config = GlConfig::default();

    // Initialize the window with the set configurations
    let mut window = Window::new(window_config, gl_config);

    let mut triangle_mesh = Mesh::empty();
    let mut color_shader = Shader::empty();

    window.set_update_callback(move |handle| {
        if handle.just_initialized() {


            // Define the vertices of one triangle with colors as rgb
            let vertices = vec![
                // positions      // colors
                0.0, 0.5, 0.0,    1.0, 0.0, 0.0, // top vertex (red)
                -0.5, -0.5, 0.0,  0.0, 1.0, 0.0, // bottom left vertex (green)
                0.5, -0.5, 0.0,   0.0, 0.0, 1.0, // bottom right vertex (blue)
            ];

            // Define the vertex attributes: position (vec3) and color (vec3)
            let attributes = vec![
                VertexAttribute::new(0, 3, 6, 0), // position (vec3)
                VertexAttribute::new(1, 3, 6, 3), // color (vec3)
            ];

            // Set up a simple triangle mesh
            triangle_mesh = Mesh::new(MeshConfig {
                vertices,
                attributes,
                ..Default::default()
            });

            // Set up the color shader
            color_shader = Shader::new_from_file(
                Path::new("./examples/assets/shaders/color.vert"),
                Path::new("./examples/assets/shaders/color.frag"),
            )
            .unwrap();
        } else {
            // Create an empty transform matrix, great for 2D rendering in our case!
            let transform = Mat4::IDENTITY;

            // Set the shader and push the transform matrix
            color_shader.bind();
            color_shader.set_uniform("transform", ShaderDataType::Mat4(transform));

            triangle_mesh.draw();
        }
    });

    window.start_event_loop();
}
