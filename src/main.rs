use crate::{gui::draw_gui, pipeline::create_pipeline, sound_generator::sound_generator};
use gst::{State, prelude::*};
use tracing::info;

mod config;
mod gui;
mod pipeline;
mod sound_generator;
mod types;

fn main() {
    tracing_subscriber::fmt::init();

    println!("#################");
    println!("### Gst-Synth ###");
    println!("#################");

    gst::init().expect("Failed to initialize Gstreamer");
    gstgtk4::plugin_register_static().expect("Failed to register gtk4 plugin");

    let main_context = glib::MainContext::default();
    let _guard = main_context.acquire().unwrap();
    let main_loop = glib::MainLoop::new(Some(&main_context), false);

    info!("Start pipeline ...");
    let pipeline = create_pipeline();
    let _ = pipeline
        .set_state(State::Playing)
        .expect("Failed to start pipeline");

    let audio_source = pipeline
        .by_name("audio_source")
        .expect("audio_source not found");
    let audio_amplify = pipeline
        .by_name("audio_amplify")
        .expect("audio_amplify not found");
    let video_sink = pipeline
        .by_name("video_sink")
        .expect("video_sink not found");
    let effect_bin = pipeline
        .by_name("effect_bin")
        .expect("effect_bin not found");

    info!("Set up sound generator ...");
    let (command_tx, command_rx) = async_channel::bounded(8);
    let main_loop_clone = main_loop.clone();
    main_context.spawn_local(async move {
        sound_generator(audio_source, audio_amplify, command_rx).await;
        main_loop_clone.quit();
    });

    info!("Draw gui ...");
    draw_gui(command_tx.clone(), video_sink, effect_bin);
    main_loop.run();

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}
