/// Caminhos de vídeo para fallback (quando câmera não estiver disponível)
pub const VIDEO_PATHS: &[&str] = &[
    "hand_video.mp4",
    "videos/hand_video.mp4",
    "test_video.mp4",
    "video.mp4",
];

/// Default audio file (will use first available from AUDIO_PATHS)
pub const DEFAULT_AUDIO_FILE: &str = "audio.mp3";

/// Pitch range constants
pub const MIN_PITCH: f32 = 0.25;  // Two octaves lower
pub const MAX_PITCH: f32 = 4.0;   // Two octaves higher
pub const DEFAULT_PITCH: f32 = 1.0; // Normal pitch