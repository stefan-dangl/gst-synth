use std::time::Duration;

// GUI
pub const GUI_TITLE: &str = "Gst - Synth";
pub const DEFAULT_WIDTH: i32 = 1600;
pub const DEFAULT_HEIGHT: i32 = 600;

// Default Settings
pub const OCTAVE_DEFAULT: i32 = 4;
pub const RELEASE_TIME_DEFAULT: Duration = Duration::from_millis(3000);
pub const ATTACK_TIME_DEFAULT: Duration = Duration::from_millis(100);
pub const WAVEFORM_DEFAULT: &str = "saw";
pub const DEFAULT_ECHO_DELAY: f64 = 550.0;
pub const DEFAULT_ECHO_INTENSITY: f64 = 0.2;
pub const DEFAULT_ECHO_FEEDBACK: f64 = 0.4;
pub const NOTE_REPEAT_INTERVAL: Duration = Duration::from_millis(5);

// Ranges
pub const OCTAVE_MIN: i32 = 1;
pub const OCTAVE_MAX: i32 = 7;
pub const MAX_AMPLIFICATION: f32 = 0.5;
