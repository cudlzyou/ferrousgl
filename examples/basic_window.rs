use ferrousgl2::*;

fn main() {
    // Initialize the default window configurations and OpenGL graphics
    let window_config = WindowConfig::default();
    let gl_config = GlConfig::default();

    // Initialize the window with the set configurations
    let mut window = Window::new(window_config, gl_config);

    // Initialize your empty(!) shaders, textures, etc with the ::empty() function
    // The OpenGL context only starts when the windows event loop starts running
    // So running OpenGL bound things will not work here, only use it for non-OpenGL related things

    // Start the event loop with a custom function after setting everything up
    // You can freely use the handle to access window functions
    window.set_update_callback(move |handle| {
        // Use this helper function to run code at the very start of the window
        if handle.just_initialized() {
            // Run code here to load a texture, setup a mesh, create a shader
        } else {
            // Here you actually run your updating code, such as rendering a mesh
        }
    });

    // Final code to start the loop you just set. Technically you don't need to do anythign
    // between this and setting the update callback
    window.start_event_loop();
}
