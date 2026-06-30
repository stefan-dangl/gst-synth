use crate::config::{
    DEFAULT_ECHO_DELAY, DEFAULT_ECHO_FEEDBACK, DEFAULT_ECHO_INTENSITY, WAVEFORM_DEFAULT,
};
use gst::{Element, ElementFactory, Pipeline, prelude::*};

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn create_effect_bin() -> Element {
    let equalizer = ElementFactory::make("equalizer-3bands")
        .name("audio_equalizer")
        .property("band0", 0.0f64)
        .property("band1", 0.0f64)
        .property("band2", 0.0f64)
        .build()
        .expect("Failed to create equalizer-3bands element");
    let band_pass = ElementFactory::make("audiochebband")
        .name("audio_band_pass")
        .property("lower-frequency", 0f32)
        .property("upper-frequency", 20000f32)
        .property("poles", 4i32)
        .build()
        .expect("Failed to create audiochebband element");
    let echo = ElementFactory::make("audioecho")
        .name("audio_echo")
        .property("max-delay", 3_000_000_000u64)
        .property("delay", (DEFAULT_ECHO_DELAY * 1_000_000.0) as u64)
        .property("intensity", DEFAULT_ECHO_INTENSITY as f32)
        .property("feedback", DEFAULT_ECHO_FEEDBACK as f32)
        .build()
        .expect("Failed to create audioecho element");

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
        .expect("Failed to create audiotestsrc element");
    let audio_amplify = ElementFactory::make("audioamplify")
        .name("audio_amplify")
        .property("amplification", 0.0f32)
        .build()
        .expect("Failed to create audioamplify element");
    let effect_bin = create_effect_bin();
    let tee = ElementFactory::make("tee")
        .name("tee")
        .build()
        .expect("Failed to create tee element");

    // Audio branch
    let audio_queue = ElementFactory::make("queue")
        .name("audio_queue")
        .property("max-size-time", 20_000_000u64)
        .build()
        .expect("Failed to create audio queue element");
    let audio_convert_audio = ElementFactory::make("audioconvert")
        .name("audio_convert")
        .build()
        .expect("Failed to create audioconvert element");
    let audio_resample = ElementFactory::make("audioresample")
        .name("audio_resample")
        .build()
        .expect("Failed to create audioresample element");
    let audio_sink = ElementFactory::make("autoaudiosink")
        .name("audio_sink")
        .build()
        .expect("Failed to create autoaudiosink element");

    // Video branch
    let video_queue = ElementFactory::make("queue")
        .name("video_queue")
        .build()
        .expect("Failed to create video queue element");
    let audio_convert_video = ElementFactory::make("audioconvert")
        .name("audio_convert_video")
        .build()
        .expect("Failed to create audioconvert (video) element");
    let visual = ElementFactory::make("monoscope")
        .name("visual")
        .build()
        .expect("Failed to create monoscope element");
    let video_convert = ElementFactory::make("videoconvert")
        .name("video_convert")
        .build()
        .expect("Failed to create videoconvert element");
    let video_sink = ElementFactory::make("gtk4paintablesink")
        .name("video_sink")
        .build()
        .expect("Failed to create gtk4paintablesink element");

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
