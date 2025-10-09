use ferrousgl2::*;
use glam::{Mat4, Vec3, Vec4};
use noise::{NoiseFn, Perlin};
use rand::Rng;
use std::path::Path;

fn main() {
    // Initialize the default window configurations and OpenGL graphics
    let window_config = WindowConfig {
        title: "Instanced Grass Example".to_string(),
        vsync: true,
        framerate: Some(60),
        ..Default::default()
    };
    let gl_config = GlConfig::default();

    // Initialize the window with the set configurations
    let mut window = Window::new(window_config, gl_config);

    let mut triangle_mesh = Mesh::empty();
    let mut grass_shader = Shader::empty();

    // Setup a random number generator and a perlin noise generator
    let mut rng = rand::rng();
    let perlin = Perlin::new(0);

    window.set_update_callback(move |handle| {
        if handle.just_initialized() {
            // Define the vertices of a single grass mesh
            // We don't need colors here, because the fragment shader does it!
            let vertices = &[
                // positions
                0.0, 4.0, 0.0, -0.2, 2.0, 0.0, 0.2, 2.0, 0.0, 0.3, 0.0, 0.0, -0.3, 0.0, 0.0,
            ];

            // Set the indices, so we only need 5 indices for 3 triangles
            let indices = &[0, 1, 2, 2, 3, 4, 1, 2, 4];

            // Define the vertex attributes: position (vec3) and color (vec3)
            let attributes = &[
                VertexAttribute::new(0, 3, 3, 0), // position (vec3)
            ];

            let mut instance_data = vec![];

            let size = 512;

            for x in -size..size {
                for y in -size..size {
                    let data = vec![
                        ShaderDataType::Mat4(
                            Mat4::from_translation(Vec3::new(
                                x as f32 * 0.5,
                                (perlin.get([x as f64 * 0.001, y as f64 * 0.001]) * 75.0
                                    + perlin.get([x as f64 * 0.01, y as f64 * 0.01]) * 12.0
                                    + perlin.get([x as f64 * 0.1, y as f64 * 0.1]) * 1.0)
                                    as f32,
                                y as f32 * 0.5,
                            )) * Mat4::from_translation(Vec3::new(
                                rng.random_range(-0.5..0.5),
                                rng.random_range(-0.1..0.1),
                                rng.random_range(-0.5..0.5),
                            )) * Mat4::from_rotation_y(rng.random_range(-45.0..45.0)),
                        ),
                        ShaderDataType::Vec3(Vec3::new(
                            0.5 - perlin.get([x as f64 * 0.01, y as f64 * 0.01]) as f32 * 0.5,
                            0.5 + perlin.get([x as f64 * 0.01, y as f64 * 0.01]) as f32 * 0.5,
                            0.1,
                        )),
                    ];
                    instance_data.push(data);
                }
            }

            // Setup a simple triangle mesh
            triangle_mesh.new_with_indices(vertices, indices, attributes);
            triangle_mesh.set_instance_attributes(&instance_data, 1);

            // Setup the color shader
            grass_shader = Shader::new_from_file(
                Path::new("./examples/assets/shaders/grass.vert"),
                Path::new("./examples/assets/shaders/grass.frag"),
            )
            .unwrap();
        } else {
            // Clear the color and depth buffers and set the OpenGL viewport to be the window size
            handle.clear_color(Vec4::new(0.0, 0.0, 0.0, 1.0));
            handle
                .set_opengl_viewport((handle.get_size().unwrap().0, handle.get_size().unwrap().1));

            let aspect_ratio = handle.get_size().unwrap().0 / handle.get_size().unwrap().1; // window width / height
            let fovy = 45.0_f32.to_radians(); // field of view in radians
            let near = 0.1;
            let far = 10000.0;

            let projection = Mat4::perspective_rh(fovy, aspect_ratio as f32, near, far);

            let frame_count = handle.get_frames_count() as f32;

            let angle = frame_count as f32 * 0.001; // rotation speed
            let radius = 32.0 + (frame_count * 0.005).sin() * 16.0; // distance from the origin
            let height = 15.0; // camera height

            let eye = Vec3::new(radius * angle.cos(), height, radius * angle.sin());
            let target = Vec3::ZERO; // look at the origin
            let up = Vec3::Y; // usually (0,1,0)

            let view = Mat4::look_at_rh(eye, target, up);

            // Create an empty transform matrix, great for 2D rendering in our case!
            let model = Mat4::from_scale(Vec3::new(0.3, 0.3, 0.3));

            // Final MVP matrix
            let mvp = projection * view * model;

            // Set the shader and push the transform matrix
            grass_shader.bind();
            grass_shader.set_uniform("transform", ShaderDataType::Mat4(mvp));
            grass_shader.set_uniform("time", ShaderDataType::Float(frame_count * 0.016)); // Run at 60fps

            triangle_mesh.draw();
        }
    });

    window.start_event_loop();
}
