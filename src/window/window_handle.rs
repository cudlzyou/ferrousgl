use glam::Vec4;
use glutin::prelude::*;
use glutin::surface::{GlSurface, Surface, WindowSurface};
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder, NotCurrentContext, Robustness, Version},
    display::GetGlDisplay,
};
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::HasWindowHandle;
use std::thread;
use std::time::{Duration, Instant};
use std::{cell::RefCell, ffi::CString, num::NonZeroU32, rc::Rc};
use winit::window::WindowButtons;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Fullscreen, Window as WinitWindow},
};

/// Window handle that can be shared with the update callback
pub struct WindowHandle {
    pub(crate) window: Option<WinitWindow>,
    pub(crate) context: Option<glutin::context::PossiblyCurrentContext>,
    pub(crate) surface: Option<Surface<WindowSurface>>,
    pub(crate) running: bool,
    pub(crate) frame_count: i32,
    pub(crate) frame_time: f32,
    pub(crate) last_frame_time: Instant,
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

    pub fn set_fullscreen(&self, fullscreen: bool) {
        if let Some(window) = &self.window {
            if fullscreen {
                let monitor = window.current_monitor();
                window.set_fullscreen(Some(Fullscreen::Borderless(monitor)));
            } else {
                window.set_fullscreen(None);
            }
        }
    }

    pub fn set_resizable(&self, resizable: bool) {
        if let Some(window) = &self.window {
            window.set_resizable(resizable);
        }
    }

    pub fn set_decorations(&self, decorations: bool) {
        if let Some(window) = &self.window {
            window.set_decorations(decorations);
        }
    }

    pub fn set_maximized(&self, maximized: bool) {
        if let Some(window) = &self.window {
            window.set_maximized(maximized);
        }
    }

    pub fn set_minimized(&self, minimized: bool) {
        if let Some(window) = &self.window {
            window.set_minimized(minimized);
        }
    }

    pub fn set_always_on_top(&self, always_on_top: bool) {
        if let Some(window) = &self.window {
            if always_on_top {
                window.set_window_level(winit::window::WindowLevel::AlwaysOnTop);
            } else {
                window.set_window_level(winit::window::WindowLevel::Normal);
            }
        }
    }

    pub fn set_position(&self, (x, y): (i32, i32)) {
        if let Some(window) = &self.window {
            window.set_outer_position(winit::dpi::LogicalPosition::new(x, y));
        }
    }

    pub fn set_size(&self, (width, height): (u32, u32)) {
        if let Some(window) = &self.window {
            //window.set_inner_size(winit::dpi::LogicalSize::new(width, height));
        }
    }

    pub fn get_frames_count(&self) -> i32 {
        self.frame_count
    }

    pub fn get_frame_time(&self) -> f32 {
        self.frame_time
    }

    /// Convinience function to be able to easily initialize shaders, load textures
    pub fn just_initialized(&self) -> bool {
        self.frame_count == 0
    }

    pub fn set_opengl_viewport(&self, (sizex, sizey): (u32, u32)) {
        if let Some(window) = &self.window {
            unsafe {
                gl::Viewport(0, 0, sizex as i32, sizey as i32);
            }
        }
    }

    pub fn clear_color(&self, color: Vec4) {
        unsafe {
            // Set the clear color
            gl::ClearColor(color.x, color.y, color.z, color.w);

            // Enable depth testing
            gl::Enable(gl::DEPTH_TEST);

            // Clear both color and depth buffers
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
}
