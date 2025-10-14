use ferrousgl2::*;
use glam::{Mat4, Vec3, Vec4};
use rand::{Rng, rng};
use std::f32::consts::PI;
use std::path::Path;
use noise::{NoiseFn, Perlin};

fn main() {
    // Initialize the default window configurations and OpenGL graphics
    let window_config = WindowConfig {
        title: "Colored Triangle Example".to_string(),
        framerate: Some(60),
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
                 0.0,   8.0, 0.0,
                -0.15,  6.0, 0.0,
                 0.15,  6.0, 0.0,
                -0.275, 4.0, 0.0,
                 0.275, 4.0, 0.0,
                -0.35,  2.0, 0.0,
                 0.35,  2.0, 0.0,
                -0.4,   0.0, 0.0,
                 0.4,   0.0, 0.0,
            ];

            let indices = vec![
                0, 1, 2, 1, 2, 3, 2, 3, 4, 3, 4, 5, 4, 5, 6, 5, 6, 7, 6, 7, 8,
            ];

            // Define the vertex attributes: position (vec3) and color (vec3)
            let attributes = vec![
                VertexAttribute::new(0, 3, 3, 0), // position (vec3)
            ];

            // Set up a simple triangle mesh
            triangle_mesh = Mesh::new(MeshConfig {
                vertices,
                indices: Some(indices),
                attributes,
                primitive_type: PrimitveType::TriangleStrip,
                ..Default::default()
            });

            let mut rng = rng();
            let mut noise = Perlin::new(32);

            let mut data: Vec<Vec<ShaderDataType>> = Vec::new(); // Or simply = vec![];

            let size = 128;

            // 2. The loop now correctly calls the 'push' method on the mutable Vec.
            for x in -size..size-1 {
                for y in -size..size-1 {
                    // We only need the loop iteration count, so use an underscore for 'i'
                    data.push(vec![
                        ShaderDataType::Vec3(Vec3::new(
                            x as f32 * 0.6 + rng.random_range(-0.45..0.45),
                            noise.get([x as f64*0.02, 0.0, y as f64*0.02]) as f32*10.0,
                            y as f32 * 0.6 + rng.random_range(-0.45..0.45),
                        )),
                        ShaderDataType::Mat4(Mat4::from_rotation_y(
                            rng.random_range(0.0..PI * 2.0)) *
                            Mat4::from_scale(Vec3::new(0.5, 1.0, 1.0))
                        ),
                        ShaderDataType::Float(
                            rng.random_range(-1.0..1.0),
                        )
                    ]);
                }
            }

            triangle_mesh.set_instance_attributes(data.as_ref(), 3);

            // Set up the color shader
            color_shader = Shader::new_from_file(
                Path::new("./examples/assets/shaders/color.vert"),
                Path::new("./examples/assets/shaders/color.frag"),
            )
            .unwrap();
        } else {
            handle.set_opengl_viewport(handle.get_size().unwrap());
            handle.clear_color(Vec4::new(0.0, 0.0, 0.0, 1.0));
            // Create an empty transform matrix, great for 2D rendering in our case!
            let model = Mat4::from_scale(Vec3::new(0.9, 0.9, 0.9));
            let projection = Mat4::perspective_rh(45.0f32, 1.0, 0.1, 1000.0);
            let view =
                Mat4::look_at_rh(Vec3::new(7.0*7.0, 4.0*7.0, 0.0), Vec3::new(0.0, 0.0, 0.0), Vec3::Y);

            let transform = projection * view * model;

            let time = handle.get_frames_count() as f32;

            // Set the shader and push the transform matrix
            color_shader.bind();
            color_shader.set_uniform("transform", ShaderDataType::Mat4(transform));
            color_shader.set_uniform("time", ShaderDataType::Float(time));

            triangle_mesh.draw();
        }
    });

    window.start_event_loop();
}
