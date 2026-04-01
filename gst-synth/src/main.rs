use anyhow::Error;
use gst::{
    Element, MessageType, SeekFlags, SeekType, State,
    event::{Seek, Step},
    prelude::*,
};
use std::{io, thread, time};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

#[derive(Clone, Copy, PartialEq, Debug)]
enum Note {
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
enum WaveForm {
    Sine,
    Square,
    Saw,
    Triangle,
    Silence, // TODO: Use between notes?
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Command {
    ChangeNote(Note),
    ChangeWaveForm(WaveForm),
    ChangeOctave(usize),
}

fn handle_keyboard(ready_tx: async_channel::Sender<Command>) {
    let _stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().keys();

    loop {
        if let Some(Ok(input)) = stdin.next() {
            let command = match input {
                Key::Char('a') => Command::ChangeNote(Note::C),
                Key::Char('w') => Command::ChangeNote(Note::CSharp),
                Key::Char('s') => Command::ChangeNote(Note::D),
                Key::Char('e') => Command::ChangeNote(Note::DSharp),
                Key::Char('d') => Command::ChangeNote(Note::E),
                Key::Char('f') => Command::ChangeNote(Note::F),
                Key::Char('t') => Command::ChangeNote(Note::FSharp),
                Key::Char('g') => Command::ChangeNote(Note::G),
                Key::Char('z' | 'y') => Command::ChangeNote(Note::GSharp), // y to support german keyboards
                Key::Char('h') => Command::ChangeNote(Note::A),
                Key::Char('u') => Command::ChangeNote(Note::ASharp),
                Key::Char('j') => Command::ChangeNote(Note::B),

                Key::Char('c') => Command::ChangeWaveForm(WaveForm::Sine),
                Key::Char('v') => Command::ChangeWaveForm(WaveForm::Square),
                Key::Char('b') => Command::ChangeWaveForm(WaveForm::Saw),
                Key::Char('n') => Command::ChangeWaveForm(WaveForm::Triangle),
                Key::Char('m') => Command::ChangeWaveForm(WaveForm::Silence),

                Key::Char('1') => Command::ChangeOctave(1),
                Key::Char('2') => Command::ChangeOctave(2),
                Key::Char('3') => Command::ChangeOctave(3),
                Key::Char('4') => Command::ChangeOctave(4),
                Key::Char('5') => Command::ChangeOctave(5),
                Key::Char('6') => Command::ChangeOctave(6),
                Key::Char('7') => Command::ChangeOctave(7),
                _ => continue,
            };
            ready_tx
                .send_blocking(command)
                .expect("failed to send data through channel");
        }
        thread::sleep(time::Duration::from_millis(5));
    }
}

fn tutorial_main() {
    if let Err(err) = gst::init() {
        eprintln!("Failed to initialize Gst: {err}");
        return;
    }

    // Get a main context...
    let main_context = glib::MainContext::default();
    // ... and make it the main context by default so that we can then have a channel to send the
    // commands we received from the terminal.
    let _guard = main_context.acquire().unwrap();

    let (command_tx, command_rx) = async_channel::bounded(5);
    // TODO: Use Async instead?
    thread::spawn(move || handle_keyboard(command_tx));

    let audio_source = gst::ElementFactory::make("audiotestsrc")
        .name("audio_source")
        .property("freq", 215.0)
        .build()
        .unwrap();
    let tee = gst::ElementFactory::make("tee")
        .name("tee")
        .build()
        .unwrap();
    let audio_queue = gst::ElementFactory::make("queue")
        .name("audio_queue")
        .build()
        .unwrap();
    let audio_convert = gst::ElementFactory::make("audioconvert")
        .name("audio_convert")
        .build()
        .unwrap();
    let audio_resample = gst::ElementFactory::make("audioresample")
        .name("audio_resample")
        .build()
        .unwrap();
    let audio_sink = gst::ElementFactory::make("autoaudiosink")
        .name("audio_sink")
        .build()
        .unwrap();
    let video_queue = gst::ElementFactory::make("queue")
        .name("video_queue")
        .build()
        .unwrap();
    let visual = gst::ElementFactory::make("wavescope")
        .name("visual")
        .property_from_str("shader", "none")
        .property_from_str("style", "lines")
        .build()
        .unwrap();
    let video_convert = gst::ElementFactory::make("videoconvert")
        .name("video_convert")
        .build()
        .unwrap();
    let video_sink = gst::ElementFactory::make("autovideosink")
        .name("video_sink")
        .build()
        .unwrap();

    let pipeline = gst::Pipeline::with_name("test-pipeline");

    pipeline
        .add_many([
            &audio_source,
            &tee,
            &audio_queue,
            &audio_convert,
            &audio_resample,
            &audio_sink,
            &video_queue,
            &visual,
            &video_convert,
            &video_sink,
        ])
        .unwrap();

    gst::Element::link_many([&audio_source, &tee]).unwrap();
    gst::Element::link_many([&audio_queue, &audio_convert, &audio_resample, &audio_sink]).unwrap();
    gst::Element::link_many([&video_queue, &visual, &video_convert, &video_sink]).unwrap();

    let tee_audio_pad = tee.request_pad_simple("src_%u").unwrap();
    println!(
        "Obtained request pad {} for audio branch",
        tee_audio_pad.name()
    );
    let queue_audio_pad = audio_queue.static_pad("sink").unwrap();
    tee_audio_pad.link(&queue_audio_pad).unwrap();

    let tee_video_pad = tee.request_pad_simple("src_%u").unwrap();
    println!(
        "Obtained request pad {} for video branch",
        tee_video_pad.name()
    );
    let queue_video_pad = video_queue.static_pad("sink").unwrap();
    tee_video_pad.link(&queue_video_pad).unwrap();

    // Start playing
    let _ = pipeline
        .set_state(State::Playing)
        .expect("Failed to start pipeline");

    let main_loop = glib::MainLoop::new(Some(&main_context), false);
    let main_loop_clone = main_loop.clone();
    let pipeline_weak = pipeline.downgrade();

    let mut octave = 4;

    main_context.spawn_local(async move {
        while let Ok(command) = command_rx.recv().await {
            let Some(pipeline) = pipeline_weak.upgrade() else {
                break;
            };
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
    });

    main_loop.run();

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    tutorial_main();
}
