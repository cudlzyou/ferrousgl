/// Window configuration options
pub struct WindowConfig {
    pub size: (u32, u32),
    pub position: (i32, i32),
    pub title: String,
    pub fullscreen: bool,
    pub decorated: bool,
    pub translucent: bool,
    pub clickthrough: bool,
    pub always_on_top: bool,
    pub hide_cursor: bool,
    pub vsync: bool,
    pub framerate: Option<u32>,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            size: (800, 600),
            position: (100, 100),
            title: "FerrousGL Window".to_string(),
            fullscreen: false,
            decorated: true,
            translucent: false,
            clickthrough: false,
            always_on_top: false,
            hide_cursor: false,
            vsync: false,
            framerate: None,
        }
    }
}