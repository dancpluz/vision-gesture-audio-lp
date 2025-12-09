/// Configurações para detecção de palma
pub struct DetectionConfig {
    pub motion_threshold: f64,
    pub min_contour_area: f64,
    pub max_contour_area: f64,
    pub min_circularity: f64,
    pub min_solidity: f64,
    pub min_aspect_ratio: f64,
    pub kernel_size: i32,
    pub morph_iterations: i32,
}

impl Default for DetectionConfig {
    fn default() -> Self {
        Self {
            motion_threshold: 25.0,
            min_contour_area: 1000.0,
            max_contour_area: 30000.0,
            min_circularity: 0.3,
            min_solidity: 0.4,
            min_aspect_ratio: 0.5,
            kernel_size: 5,
            morph_iterations: 1,
        }
    }
}

/// Constantes da UI
pub mod ui_constants {
    pub const CROSS_SIZE: i32 = 25;
    pub const INDICATOR_RADIUS: i32 = 15;
    pub const INDICATOR_X_OFFSET: i32 = 30;
    pub const INDICATOR_Y_OFFSET: i32 = 30;
    pub const CENTER_CIRCLE_RADIUS: i32 = 8;
}

/// Caminhos de vídeo para fallback
pub const VIDEO_PATHS: &[&str] = &[
    "hand_video.mp4",
    "videos/hand_video.mp4",
    "test_video.mp4",
];