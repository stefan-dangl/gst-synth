use std::time::Duration;

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
