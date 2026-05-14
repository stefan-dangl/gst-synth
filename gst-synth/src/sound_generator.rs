use crate::types::{Command, Note, OCTAVE_DEFAULT, WaveForm};
use glib::MainContext;
use gst::{Element, prelude::*};
use std::time::Duration;

const RELEASE_TIME: Duration = Duration::from_millis(50);

pub async fn sound_generator(
    audio_source: Element,
    audio_amplify: Element,
    command_rx: async_channel::Receiver<Command>,
    main_context: MainContext,
) {
    let mut octave = OCTAVE_DEFAULT;
    let mut wave_form = "sine";
    let mut note_release_task: Option<glib::JoinHandle<()>> = None;

    while let Ok(command) = command_rx.recv().await {
        match command {
            Command::Quit => {
                return;
            }

            Command::ChangeNote(note) => {
                if let Some(task) = note_release_task.take() {
                    task.abort();
                }

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
                audio_source.set_property_from_str("wave", wave_form); // TODO_SD: Move
                audio_amplify.set_property("amplification", 1.0f32);
                audio_source.set_property("freq", freq * 2.0_f64.powi(octave as i32));
                note_release_task
                    .replace(main_context.spawn_local(note_release(audio_amplify.clone())));
            }

            Command::ChangeWaveForm(wave) => {
                wave_form = match wave {
                    WaveForm::Sine => "sine",
                    WaveForm::Square => "square",
                    WaveForm::Saw => "saw",
                    WaveForm::Triangle => "triangle",
                };
            }

            Command::ChangeOctave(value) => {
                octave = value;
            }
        };
    }
}

async fn note_release(audio_amplify: Element) {
    // TODO: make configurable
    const FADE_OUT_TIME: Duration = Duration::from_millis(1000);
    const FADE_OUT_STEPS: u32 = 100;
    let fade_out_sleep_time = FADE_OUT_TIME / FADE_OUT_STEPS;

    glib::timeout_future(RELEASE_TIME).await;
    println!("Release time passed");

    let mut amplification = 1.0f32;
    while amplification > 0.0 {
        amplification -= 1.0 / FADE_OUT_STEPS as f32;
        amplification = amplification.max(0.0);
        println!("!!! AMPLIFICATION: {amplification}");
        audio_amplify.set_property("amplification", amplification);
        glib::timeout_future(fade_out_sleep_time).await;
    }
}
