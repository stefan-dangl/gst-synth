use crate::{gui::draw_gui, pipeline::create_pipeline, processor::process};
use gst::{State, prelude::*};

mod gui;
mod keyboard;
mod pipeline;
mod processor;
mod types;

fn main() {
    gst::init().expect("Failed to initialize Gstreamer");
    gstgtk4::plugin_register_static().expect("Failed to register gtk4 plugin");
    let main_context = glib::MainContext::default();
    let _guard = main_context.acquire().unwrap();
    let main_loop = glib::MainLoop::new(Some(&main_context), false);

    let pipeline = create_pipeline();
    let _ = pipeline
        .set_state(State::Playing)
        .expect("Failed to start pipeline");

    let audio_source = pipeline
        .by_name("audio_source")
        .expect("audio_source not found");
    let video_sink = pipeline
        .by_name("video_sink")
        .expect("video_sink not found");
    let effect_bin = pipeline
        .by_name("effect_bin")
        .expect("effect_bin not found");

    let (command_tx, command_rx) = async_channel::bounded(5);
    let main_loop_clone = main_loop.clone();
    let main_context_clone = main_context.clone();
    main_context.spawn_local(async move {
        process(audio_source, command_rx, main_context_clone).await;
        main_loop_clone.quit();
    });

    draw_gui(command_tx.clone(), video_sink, effect_bin);
    main_loop.run();

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}
