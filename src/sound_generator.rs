use crate::config::{
    ATTACK_TIME_DEFAULT, MAX_AMPLIFICATION, NOTE_REPEAT_INTERVAL, OCTAVE_DEFAULT,
    RELEASE_TIME_DEFAULT, WAVEFORM_DEFAULT,
};
use crate::types::{Command, Note, Setting, WaveForm};
use glib::MainContext;
use gst::{Element, prelude::*};
use std::time::Duration;
use tracing::{debug, info};

const SUSTAIN_TIME: Duration = Duration::from_millis(50);
const RELEASE_STEPS: u32 = 100;

pub async fn sound_generator(
    audio_source: Element,
    audio_amplify: Element,
    command_rx: async_channel::Receiver<Command>,
) {
    let main_context = MainContext::default();
    sound_generator_inner(audio_source, audio_amplify, command_rx, main_context).await;
}

async fn sound_generator_inner(
    audio_source: Element,
    audio_amplify: Element,
    command_rx: async_channel::Receiver<Command>,
    main_context: MainContext,
) {
    let mut octave = OCTAVE_DEFAULT;
    let mut last_wave_form = WAVEFORM_DEFAULT;
    let mut note_release_task: Option<glib::JoinHandle<()>> = None;
    let mut release_time = RELEASE_TIME_DEFAULT;
    let mut attack_time = ATTACK_TIME_DEFAULT;

    while let Ok(command) = command_rx.recv().await {
        match command {
            Command::Quit => {
                return;
            }

            Command::ChangeNote(note) => {
                debug!(?note, "Note played");

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
                audio_source.set_property("freq", freq * 2.0_f64.powi(octave));
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
                if wave_form != last_wave_form {
                    info!(?wave_form, "Waveform changed");
                    last_wave_form = wave_form;
                    audio_source.set_property_from_str("wave", wave_form);
                }
            }

            Command::ChangeOctave(value) => {
                if value != octave {
                    info!(?value, "Octave changed");
                    octave = value;
                }
            }

            Command::ChangeSetting(setting) => match setting {
                Setting::AttackTime(duration) => attack_time = duration,
                Setting::ReleaseTime(duration) => release_time = duration,
            },
        }
    }
}

fn note_attack(audio_amplify: &Element, attack_time: Duration) {
    let x_steps = attack_time.as_secs_f32() / NOTE_REPEAT_INTERVAL.as_secs_f32();
    let y_steps = MAX_AMPLIFICATION / x_steps;

    let amplification = audio_amplify.property::<f32>("amplification");
    if amplification < MAX_AMPLIFICATION {
        let amplification_new = (amplification + y_steps).min(MAX_AMPLIFICATION);
        audio_amplify.set_property("amplification", amplification_new);
    }
}

async fn note_release(audio_amplify: Element, release_time: Duration) {
    let fade_out_sleep_time = release_time / RELEASE_STEPS;

    glib::timeout_future(SUSTAIN_TIME).await;

    let mut amplification = audio_amplify.property::<f32>("amplification");
    #[allow(clippy::cast_precision_loss)]
    while amplification > 0.0 {
        amplification -= 1.0 / RELEASE_STEPS as f32;
        amplification = amplification.max(0.0);
        audio_amplify.set_property("amplification", amplification);
        glib::timeout_future(fade_out_sleep_time).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Command, Note, WaveForm};
    use gst::{Element, ElementFactory, Pipeline};
    use std::future::Future;
    use std::time::Duration;

    fn setup_test_pipeline() -> (Pipeline, Element, Element) {
        gst::init().unwrap();
        let src = ElementFactory::make("audiotestsrc")
            .name("audio_source")
            .property("is-live", false)
            .build()
            .unwrap();
        let amp = ElementFactory::make("audioamplify")
            .name("audio_amplify")
            .property("amplification", 0.0f32)
            .build()
            .unwrap();
        let sink = ElementFactory::make("fakesink")
            .property("sync", false)
            .build()
            .unwrap();
        let pipeline = Pipeline::new();
        pipeline.add_many([&src, &amp, &sink]).unwrap();
        Element::link_many([&src, &amp, &sink]).unwrap();
        pipeline.set_state(gst::State::Playing).unwrap();
        (pipeline, src, amp)
    }

    fn read_waveform(source: &Element) -> String {
        use glib::translate::ToGlibPtr;
        let val = source.property_value("wave");
        let enum_class = glib::EnumClass::with_type(val.type_()).unwrap();
        // g_value_get_enum is the correct C getter for GEnum values; no safe Rust API exists
        // for reading an arbitrary GEnum without a generated wrapper type.
        let int_val =
            unsafe { glib::gobject_ffi::g_value_get_enum(val.to_glib_none().0 as *mut _) };
        enum_class.value(int_val).unwrap().nick().to_string()
    }

    fn run<F, Fut>(f: F)
    where
        F: FnOnce(glib::MainContext) -> Fut,
        Fut: Future<Output = ()> + 'static,
    {
        let ctx = MainContext::new();
        let fut = f(ctx.clone());
        ctx.block_on(fut);
    }

    #[test]
    fn change_waveform() {
        run(|ctx| async move {
            let (pipeline, src, amp) = setup_test_pipeline();
            let (tx, rx) = async_channel::bounded(5);
            ctx.spawn_local(sound_generator_inner(src.clone(), amp, rx, ctx.clone()));

            for (wave, expected_nick) in [
                (WaveForm::Sine, "sine"),
                (WaveForm::Square, "square"),
                (WaveForm::Saw, "saw"),
                (WaveForm::Triangle, "triangle"),
            ] {
                tx.send(Command::ChangeWaveForm(wave)).await.unwrap();
                glib::timeout_future(Duration::from_millis(10)).await;
                assert_eq!(
                    read_waveform(&src),
                    expected_nick,
                    "waveform mismatch for {wave:?}"
                );
            }

            tx.send(Command::Quit).await.unwrap();
            glib::timeout_future(Duration::from_millis(10)).await;
            pipeline.set_state(gst::State::Null).unwrap();
        });
    }

    #[test]
    fn change_note() {
        run(|ctx| async move {
            let (pipeline, src, amp) = setup_test_pipeline();
            let (tx, rx) = async_channel::bounded(5);
            ctx.spawn_local(sound_generator_inner(src.clone(), amp, rx, ctx.clone()));
            tx.send(Command::ChangeOctave(4)).await.unwrap();

            for (note, expected_freq) in [
                (Note::C, 261.63),
                (Note::CSharp, 277.18),
                (Note::D, 293.66),
                (Note::DSharp, 311.13),
                (Note::E, 329.63),
                (Note::F, 349.23),
                (Note::FSharp, 369.99),
                (Note::G, 392.0),
                (Note::GSharp, 415.3),
                (Note::A, 440.0),
                (Note::ASharp, 466.16),
                (Note::B, 493.88),
            ] {
                tx.send(Command::ChangeNote(note)).await.unwrap();
                glib::timeout_future(Duration::from_millis(10)).await;
                let freq = src.property::<f64>("freq");
                assert!(
                    (freq - expected_freq).abs() < 0.1,
                    "expected {note:?}4 = {expected_freq} Hz, got {freq}"
                );
            }

            tx.send(Command::Quit).await.unwrap();
            glib::timeout_future(Duration::from_millis(10)).await;
            pipeline.set_state(gst::State::Null).unwrap();
        });
    }

    #[test]
    fn change_octave() {
        run(|ctx| async move {
            let (pipeline, src, amp) = setup_test_pipeline();
            let (tx, rx) = async_channel::bounded(5);
            ctx.spawn_local(sound_generator_inner(src.clone(), amp, rx, ctx.clone()));

            for (octave, expected_freq) in [
                (1, 55.0),
                (2, 110.0),
                (3, 220.0),
                (4, 440.0),
                (5, 880.0),
                (6, 1760.0),
                (7, 3520.0),
                (8, 7040.0),
            ] {
                tx.send(Command::ChangeOctave(octave)).await.unwrap();
                tx.send(Command::ChangeNote(Note::A)).await.unwrap();
                glib::timeout_future(Duration::from_millis(10)).await;
                let freq = src.property::<f64>("freq");
                assert!(
                    (freq - expected_freq).abs() < 0.1,
                    "expected A{octave} = {expected_freq} Hz, got {freq}"
                );
            }

            tx.send(Command::Quit).await.unwrap();
            glib::timeout_future(Duration::from_millis(10)).await;
            pipeline.set_state(gst::State::Null).unwrap();
        });
    }
}
