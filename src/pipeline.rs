use gst::{Element, ElementFactory, Pipeline, prelude::*};

use crate::types::{
    DEFAULT_ECHO_DELAY, DEFAULT_ECHO_FEEDBACK, DEFAULT_ECHO_INTENSITY, WAVEFORM_DEFAULT,
};

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn create_effect_bin() -> Element {
    let equalizer = ElementFactory::make("equalizer-3bands")
        .name("audio_equalizer")
        .property("band0", 0.0f64)
        .property("band1", 0.0f64)
        .property("band2", 0.0f64)
        .build()
        .unwrap();
    let band_pass = ElementFactory::make("audiochebband")
        .name("audio_band_pass")
        .property("lower-frequency", 0f32)
        .property("upper-frequency", 20000f32)
        .property("poles", 4i32)
        .build()
        .unwrap();
    let echo = ElementFactory::make("audioecho")
        .name("audio_echo")
        .property("max-delay", 3_000_000_000u64)
        .property("delay", (DEFAULT_ECHO_DELAY * 1_000_000.0) as u64)
        .property("intensity", DEFAULT_ECHO_INTENSITY as f32)
        .property("feedback", DEFAULT_ECHO_FEEDBACK as f32)
        .build()
        .unwrap();

    let bin = gst::Bin::with_name("effect_bin");
    bin.add_many([&equalizer, &band_pass, &echo]).unwrap();
    Element::link_many([&equalizer, &band_pass, &echo]).unwrap();
    bin.add_pad(&gst::GhostPad::with_target(&equalizer.static_pad("sink").unwrap()).unwrap())
        .unwrap();
    bin.add_pad(&gst::GhostPad::with_target(&echo.static_pad("src").unwrap()).unwrap())
        .unwrap();

    bin.upcast::<Element>()
}

pub fn create_pipeline() -> Pipeline {
    // Audio Source
    let audio_source = ElementFactory::make("audiotestsrc")
        .name("audio_source")
        .property("is-live", true)
        .property_from_str("wave", WAVEFORM_DEFAULT)
        .build()
        .unwrap();
    let audio_amplify = ElementFactory::make("audioamplify")
        .name("audio_amplify")
        .property("amplification", 0.0f32)
        .build()
        .unwrap();
    let effect_bin = create_effect_bin();
    let tee = ElementFactory::make("tee").name("tee").build().unwrap();

    // Audio branch
    let audio_queue = ElementFactory::make("queue")
        .name("audio_queue")
        .property("max-size-time", 20_000_000u64)
        .build()
        .unwrap();
    let audio_convert_audio = ElementFactory::make("audioconvert")
        .name("audio_convert")
        .build()
        .unwrap();
    let audio_resample = ElementFactory::make("audioresample")
        .name("audio_resample")
        .build()
        .unwrap();
    let audio_sink = ElementFactory::make("autoaudiosink")
        .name("audio_sink")
        .build()
        .unwrap();

    // Video branch
    let video_queue = ElementFactory::make("queue")
        .name("video_queue")
        .build()
        .unwrap();
    let audio_convert_video = ElementFactory::make("audioconvert")
        .name("audio_convert_video")
        .build()
        .unwrap();
    let visual = ElementFactory::make("monoscope")
        .name("visual")
        .build()
        .unwrap();
    let video_convert = ElementFactory::make("videoconvert")
        .name("video_convert")
        .build()
        .unwrap();
    let video_sink = ElementFactory::make("gtk4paintablesink")
        .name("video_sink")
        .build()
        .unwrap();

    let pipeline = Pipeline::with_name("synth-pipeline");

    pipeline
        .add_many([
            &audio_source,
            &audio_amplify,
            &effect_bin,
            &tee,
            &audio_queue,
            &audio_convert_audio,
            &audio_resample,
            &audio_sink,
            &video_queue,
            &audio_convert_video,
            &visual,
            &video_convert,
            &video_sink,
        ])
        .unwrap();

    Element::link_many([&audio_source, &audio_amplify, &effect_bin, &tee]).unwrap();
    Element::link_many([
        &audio_queue,
        &audio_convert_audio,
        &audio_resample,
        &audio_sink,
    ])
    .unwrap();
    Element::link_many([
        &video_queue,
        &audio_convert_video,
        &visual,
        &video_convert,
        &video_sink,
    ])
    .unwrap();

    let tee_audio_pad = tee.request_pad_simple("src_%u").unwrap();
    let queue_audio_pad = audio_queue.static_pad("sink").unwrap();
    tee_audio_pad.link(&queue_audio_pad).unwrap();

    let tee_video_pad = tee.request_pad_simple("src_%u").unwrap();
    let queue_video_pad = video_queue.static_pad("sink").unwrap();
    tee_video_pad.link(&queue_video_pad).unwrap();

    pipeline
}
