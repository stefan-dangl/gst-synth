use crate::{
    keyboard::REPEAT_INTERVAL,
    types::{
        ATTACK_TIME_DEFAULT, Command, MAX_AMPLIFICATION, Note, OCTAVE_DEFAULT,
        RELEASE_TIME_DEFAULT, Setting, WaveForm,
    },
};
use glib::MainContext;
use gst::{Element, prelude::*};
use std::time::Duration;

const SUSTAIN_TIME: Duration = Duration::from_millis(50);
const RELEASE_STEPS: u32 = 100;

pub async fn sound_generator(
    audio_source: Element,
    audio_amplify: Element,
    command_rx: async_channel::Receiver<Command>,
    main_context: MainContext,
) {
    let mut octave = OCTAVE_DEFAULT;
    let mut note_release_task: Option<glib::JoinHandle<()>> = None;
    let mut release_time = RELEASE_TIME_DEFAULT;
    let mut attack_time = ATTACK_TIME_DEFAULT;

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
                note_attack(&audio_amplify, attack_time);
                audio_source.set_property("freq", freq * 2.0_f64.powi(octave as i32));
                note_release_task.replace(
                    main_context.spawn_local(note_release(audio_amplify.clone(), release_time)),
                );
            }

            Command::ChangeWaveForm(wave) => {
                let wave_form = match wave {
                    WaveForm::Sine => "sine",
                    WaveForm::Square => "square",
                    WaveForm::Saw => "saw",
                    WaveForm::Triangle => "triangle",
                };
                audio_source.set_property_from_str("wave", wave_form);
            }

            Command::ChangeOctave(value) => {
                octave = value;
            }

            Command::ChangeSetting(setting) => match setting {
                Setting::AttackTime(duration) => attack_time = duration,
                Setting::ReleaseTime(duration) => release_time = duration,
            },
        };
    }
}

fn note_attack(audio_amplify: &Element, attack_time: Duration) {
    let x_steps = attack_time.as_nanos() / REPEAT_INTERVAL.as_nanos();
    let y_steps = MAX_AMPLIFICATION / x_steps as f32;

    let amplification = audio_amplify.property::<f32>("amplification") as f32;
    println!("!!! READ AMP: {amplification}");
    if amplification < MAX_AMPLIFICATION {
        let amplification_new = (amplification + y_steps).min(MAX_AMPLIFICATION);
        audio_amplify.set_property("amplification", amplification_new);
    }
}

async fn note_release(audio_amplify: Element, release_time: Duration) {
    let fade_out_sleep_time = release_time / RELEASE_STEPS;

    glib::timeout_future(SUSTAIN_TIME).await;
    println!("Release time passed");

    let mut amplification = audio_amplify.property::<f32>("amplification") as f32;
    while amplification > 0.0 {
        amplification -= 1.0 / RELEASE_STEPS as f32;
        amplification = amplification.max(0.0);
        println!("!!! AMPLIFICATION: {amplification}");
        audio_amplify.set_property("amplification", amplification);
        glib::timeout_future(fade_out_sleep_time).await;
    }
}
