use crate::types::{Command, Note, WaveForm};
use gst::{Element, prelude::*};

pub async fn process(audio_source: Element, command_rx: async_channel::Receiver<Command>) {
    let mut octave = 4;

    while let Ok(command) = command_rx.recv().await {
        match command {
            Command::ChangeNote(note) => {
                let freq = match note {
                    Note::C => 16.35,
                    Note::CSharp => 17.32,
                    Note::D => 18.35,
                    Note::DSharp => 19.45,
                    Note::E => 20.6,
                    Note::F => 21.83,
                    Note::FSharp => 23.12,
                    Note::G => 24.5,
                    Note::GSharp => 25.96,
                    Note::A => 27.5,
                    Note::ASharp => 29.14,
                    Note::B => 30.87,
                };
                audio_source.set_property("freq", freq * 2.0_f64.powi(octave));
            }
            Command::ChangeWaveForm(wave_form) => {
                let wave = match wave_form {
                    WaveForm::Sine => "sine",
                    WaveForm::Square => "square",
                    WaveForm::Saw => "saw",
                    WaveForm::Triangle => "triangle",
                    WaveForm::Silence => "silence",
                };
                audio_source.set_property_from_str("wave", wave);
            }

            Command::ChangeOctave(value) => {
                octave = value as i32;
            }
        };
    }
}
