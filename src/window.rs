use glutin::prelude::*;
use glutin::surface::{GlSurface, Surface, WindowSurface};
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder, NotCurrentContext, Robustness, Version},
    display::GetGlDisplay,
};
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::HasWindowHandle;
use std::{ffi::CString, num::NonZeroU32, rc::Rc, cell::RefCell};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Fullscreen, Window as WinitWindow, WindowAttributes},
};

/// Window configuration options
pub struct WindowConfig {
    pub size: (u32, u32),
    pub title: String,
    pub fullscreen: bool,
    pub decorated: bool,
    pub translucent: bool,
    pub hide_cursor: bool,
    pub vsync: bool,
    pub framerate: Option<u32>,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            size: (800, 600),
            title: "FerrousGL Window".to_string(),
            fullscreen: false,
            decorated: true,
            translucent: false,
            hide_cursor: false,
            vsync: false,
            framerate: None,
        }
    }
}

/// Graphics (OpenGL) configuration options
pub struct GlConfig {
    pub version_major: u8,
    pub version_minor: u8,
    pub robustness: Robustness,
}

impl Default for GlConfig {
    fn default() -> Self {
        Self {
            version_major: 3,
            version_minor: 3,
            robustness: Robustness::RobustLoseContextOnReset,
        }
    }
}

/// Window handle that can be shared with the update callback
pub struct WindowHandle {
    window: Option<WinitWindow>,
    context: Option<glutin::context::PossiblyCurrentContext>,
    surface: Option<Surface<WindowSurface>>,
    running: bool,
}

impl WindowHandle {
    /// Get a reference to the underlying winit window
    pub fn get_window(&self) -> Option<&WinitWindow> {
        self.window.as_ref()
    }

    /// Check if the window is still running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Request a redraw
    pub fn request_redraw(&self) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    /// Get the current window size
    pub fn get_size(&self) -> Option<(u32, u32)> {
        self.window.as_ref().map(|w| {
            let size = w.inner_size();
            (size.width, size.height)
        })
    }

    /// Set window title
    pub fn set_title(&self, title: &str) {
        if let Some(window) = &self.window {
            window.set_title(title);
        }
    }

    /// Set cursor visibility
    pub fn set_cursor_visible(&self, visible: bool) {
        if let Some(window) = &self.window {
            window.set_cursor_visible(visible);
        }
    }
}

pub struct Window {
    handle: Rc<RefCell<WindowHandle>>,
    window_config: WindowConfig,
    gl_config: GlConfig,
    user_update: Option<Box<dyn FnMut(&WindowHandle) + 'static>>,
    gl_loaded: bool,
}

impl Window {
    /// Create a new window with the given configuration
    /// Note: The window and OpenGL context will be created when the event loop starts
    pub fn new(window_config: WindowConfig, gl_config: GlConfig) -> Self {
        let handle = Rc::new(RefCell::new(WindowHandle {
            window: None,
            context: None,
            surface: None,
            running: true,
        }));

        Self {
            handle,
            window_config,
            gl_config,
            user_update: None,
            gl_loaded: false,
        }
    }

    fn init_gl(&mut self, event_loop: &ActiveEventLoop) {
        // Window attributes
        let mut attrs = WindowAttributes::default()
            .with_title(self.window_config.title.clone())
            .with_inner_size(LogicalSize::new(
                self.window_config.size.0,
                self.window_config.size.1,
            ))
            .with_decorations(self.window_config.decorated)
            .with_transparent(self.window_config.translucent);

        if self.window_config.fullscreen {
            attrs = attrs.with_fullscreen(Some(Fullscreen::Borderless(None)));
        }

        // Display builder
        let display_builder = DisplayBuilder::new().with_window_attributes(Some(attrs));

        // GL config template
        let template = ConfigTemplateBuilder::new();

        let (window, gl_config) = display_builder
            .build(event_loop, template, |mut configs| configs.next().unwrap())
            .unwrap();

        let window = window.unwrap();

        if self.window_config.hide_cursor {
            window.set_cursor_visible(false);
        }

        let window_handle = window.window_handle().unwrap().as_raw();

        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version {
                major: self.gl_config.version_major,
                minor: self.gl_config.version_minor,
            })))
            .with_robustness(self.gl_config.robustness)
            .build(Some(window_handle));

        let not_current_context: NotCurrentContext = unsafe {
            match gl_config.display().create_context(&gl_config, &context_attributes) {
                Ok(ctx) => ctx,
                Err(e) => {
                    eprintln!(
                        "Failed to create OpenGL context! Driver rejected OpenGL version: {}.{}\n\
                        Your video card might be too old to support this version.\n\
                        Please update your graphics driver.\n\nError: {}",
                        self.gl_config.version_major,
                        self.gl_config.version_minor,
                        e
                    );
                    panic!("The application cannot continue without an OpenGL context.");
                }
            }
        };

        let surface_attrs = window.build_surface_attributes(Default::default()).unwrap();

        let surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &surface_attrs)
                .unwrap()
        };

        let context = not_current_context.make_current(&surface).unwrap();

        // Load GL functions
        if !self.gl_loaded {
            gl::load_with(|symbol| {
                let symbol_cstring = CString::new(symbol).unwrap();
                context.display().get_proc_address(&symbol_cstring)
            });
            self.gl_loaded = true;
        }

        // Store window, context, and surface in the handle
        let mut handle = self.handle.borrow_mut();
        handle.window = Some(window);
        handle.context = Some(context);
        handle.surface = Some(surface);
    }

    /// Set the update callback function
    /// The callback receives a reference to WindowHandle for accessing window properties
    pub fn set_update_callback<F>(&mut self, callback: F)
    where
        F: FnMut(&WindowHandle) + 'static,
    {
        self.user_update = Some(Box::new(callback));
    }

    /// Start the main event loop and run the window
    pub fn start_event_loop(mut self) {
        // Create the single event loop
        let event_loop = EventLoop::new().expect("Failed to create event loop");
        event_loop.set_control_flow(ControlFlow::Wait);
        
        event_loop.run_app(&mut self).unwrap();
    }

    /// Convenience function to create and run a window in one call
    pub fn run<F>(window_config: WindowConfig, gl_config: GlConfig, user_update: F)
    where
        F: FnMut(&WindowHandle) + 'static,
    {
        let mut window = Self::new(window_config, gl_config);
        window.set_update_callback(user_update);
        window.start_event_loop();
    }

    fn resize(&self, width: u32, height: u32) {
        let handle = self.handle.borrow();
        if let (Some(context), Some(surface)) = (&handle.context, &handle.surface) {
            surface.resize(
                context,
                NonZeroU32::new(width).unwrap(),
                NonZeroU32::new(height).unwrap(),
            );
        }
    }

    pub fn update(&mut self) {
        let handle = self.handle.borrow();
        
        // Call user code if callback is set, passing the window handle
        if let Some(ref mut callback) = self.user_update {
            callback(&*handle);
        }

        // Swap buffers
        if let (Some(context), Some(surface)) = (&handle.context, &handle.surface) {
            surface.swap_buffers(context).unwrap();
        }

        // Request redraw
        if let Some(window) = &handle.window {
            window.request_redraw();
        }
    }

    /// Get a reference to the window handle
    pub fn get_handle(&self) -> Rc<RefCell<WindowHandle>> {
        self.handle.clone()
    }

    /// Check if OpenGL has been loaded
    pub fn is_gl_loaded(&self) -> bool {
        self.gl_loaded
    }
}

impl ApplicationHandler for Window {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Initialize the window and OpenGL context only once
        if self.handle.borrow().window.is_none() {
            self.init_gl(event_loop);

            // Request initial draw
            let handle = self.handle.borrow();
            if let Some(window) = &handle.window {
                window.request_redraw();
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                self.handle.borrow_mut().running = false;
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.update();
            }
            WindowEvent::Resized(size) => {
                self.resize(size.width, size.height);
            }
            _ => {}
        }
    }
}