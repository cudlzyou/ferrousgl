use ferrousgl2::*;

fn main() {
    let window_config = WindowConfig::default();
    let gl_config = GlConfig::default();

    // GL context is created and loaded immediately
    let mut window = Window::new(window_config, gl_config);

    // The callback now receives a WindowHandle reference
    window.set_update_callback(|handle| {
        // Access window properties
        if let Some(size) = handle.get_size() {
            println!("Window size: {}x{}", size.0, size.1);
        }

        // Can request redraws, change title, etc.
        handle.set_title("Frame rendered!");

        // Your GL rendering code here
        unsafe {
            gl::ClearColor(0.1, 0.2, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    });

    window.start_event_loop();
}
