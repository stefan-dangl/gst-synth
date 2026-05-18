use std::time::Duration;

// GUI - Related
pub const GUI_TITLE: &str = "Gst - Synth";

// Default Settings
pub const OCTAVE_DEFAULT: usize = 4;
pub const RELEASE_TIME_DEFAULT: Duration = Duration::from_millis(3000);
pub const ATTACK_TIME_DEFAULT: Duration = Duration::from_millis(100);
pub const WAVEFORM_DEFAULT: &str = "saw";
pub const DEFAULT_ECHO_DELAY: f64 = 550.0;
pub const DEFAULT_ECHO_INTENSITY: f64 = 0.2;
pub const DEFAULT_ECHO_FEEDBACK: f64 = 0.4;

// Ranges
pub const OCTAVE_MIN: usize = 1;
pub const OCTAVE_MAX: usize = 7;
pub const MAX_AMPLIFICATION: f32 = 0.5;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Note {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum WaveForm {
    Sine,
    Square,
    #[default]
    Saw,
    Triangle,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Setting {
    AttackTime(Duration),
    ReleaseTime(Duration),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Command {
    ChangeNote(Note),
    ChangeWaveForm(WaveForm),
    ChangeOctave(usize),
    ChangeSetting(Setting),
    Quit,
}
