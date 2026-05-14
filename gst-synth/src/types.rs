use std::time::Duration;

// Default Settings
pub const OCTAVE_DEFAULT: usize = 4;
pub const RELEASE_TIME_DEFAULT: Duration = Duration::from_millis(3000);
pub const ATTACK_TIME_DEFAULT: Duration = Duration::from_millis(100);
pub const WAVEFORM_DEFAULT: &str = "sine";

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

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum WaveForm {
    Sine,
    Square,
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
