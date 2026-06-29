use std::time::Duration;

// TODO_SD: Move to dedicated config module?

// GUI - Related
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
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
    ChangeOctave(i32),
    ChangeSetting(Setting),
    Quit,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum UiEvent {
    NoteChanged(Option<Note>),
    OctaveChanged(i32),
    WaveFormChanged(WaveForm),
}
