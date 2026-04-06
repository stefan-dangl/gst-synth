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
    Silence, // TODO: Use between notes?
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Command {
    ChangeNote(Note),
    ChangeWaveForm(WaveForm),
    ChangeOctave(usize),
    Quit,
}
