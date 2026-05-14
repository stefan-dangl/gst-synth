pub const OCTAVE_MIN: usize = 1;
pub const OCTAVE_MAX: usize = 7;
pub const OCTAVE_DEFAULT: usize = 4;

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
pub enum Command {
    ChangeNote(Note),
    ChangeWaveForm(WaveForm),
    ChangeOctave(usize),
    Quit,
}
