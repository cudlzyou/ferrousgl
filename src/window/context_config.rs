use glutin::context::Robustness;

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