use crate::{keyboard::handle_keyboard, pipeline::create_pipeline, processor::process};
use gst::{State, prelude::*};
use std::thread;

mod keyboard;
mod pipeline;
mod processor;
mod types;

fn main() {
    gst::init().expect("Failed to initialize Gstreamer");
    let main_context = glib::MainContext::default();
    let _guard = main_context.acquire().unwrap();
    let main_loop = glib::MainLoop::new(Some(&main_context), false);

    let (command_tx, command_rx) = async_channel::bounded(5);
    thread::spawn(move || handle_keyboard(command_tx));

    let pipeline = create_pipeline();
    let _ = pipeline
        .set_state(State::Playing)
        .expect("Failed to start pipeline");

    let audio_source = pipeline
        .by_name("audio_source")
        .expect("audio_source not found");

    main_context.spawn_local(process(audio_source, command_rx));
    main_loop.run();

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}
