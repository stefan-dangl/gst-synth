use gst::{Pipeline, prelude::*};

use crate::types::{
    DEFAULT_ECHO_DELAY, DEFAULT_ECHO_FEEDBACK, DEFAULT_ECHO_INTENSITY, WAVEFORM_DEFAULT,
};

pub fn create_pipeline() -> Pipeline {
    // Audio Source
    let audio_source = gst::ElementFactory::make("audiotestsrc")
        .name("audio_source")
        .property("is-live", true)
        .property_from_str("wave", WAVEFORM_DEFAULT)
        .build()
        .unwrap();
    let audio_amplify = gst::ElementFactory::make("audioamplify")
        .name("audio_amplify")
        .property("amplification", 0.0f32)
        .build()
        .unwrap();

    // Audio effects
    // Equalizer bands: band0 = 100Hz, band1 = 1100Hz, band2 = 11kHz, value = [-24;12]
    let equalizer = gst::ElementFactory::make("equalizer-3bands")
        .name("audio_equalizer")
        .property("band0", 0.0f64)
        .property("band1", 0.0f64)
        .property("band2", 0.0f64)
        .build()
        .unwrap();
    let band_pass = gst::ElementFactory::make("audiochebband")
        .name("audio_band_pass")
        .property("lower-frequency", 0f32)
        .property("upper-frequency", 20000f32)
        .property("poles", 4i32)
        .build()
        .unwrap();
    let echo = gst::ElementFactory::make("audioecho")
        .name("audio_echo")
        .property("max-delay", 3_000_000_000u64)
        .property("delay", (DEFAULT_ECHO_DELAY * 1_000_000.0) as u64)
        .property("intensity", DEFAULT_ECHO_INTENSITY as f32)
        .property("feedback", DEFAULT_ECHO_FEEDBACK as f32)
        .build()
        .unwrap();
    let audio_convert = gst::ElementFactory::make("audioconvert")
        .name("audio_convert")
        .build()
        .unwrap();

    let tee = gst::ElementFactory::make("tee")
        .name("tee")
        .build()
        .unwrap();

    // Audio branch
    let audio_queue = gst::ElementFactory::make("queue")
        .name("audio_queue")
        .property("max-size-time", 20_000_000u64)
        .build()
        .unwrap();
    let audio_convert_2 = gst::ElementFactory::make("audioconvert")
        .name("audio_convert_2")
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

    // Video branch
    let video_queue = gst::ElementFactory::make("queue")
        .name("video_queue")
        .build()
        .unwrap();
    let visual = gst::ElementFactory::make("monoscope")
        .name("visual")
        .build()
        .unwrap();
    let video_convert = gst::ElementFactory::make("videoconvert")
        .name("video_convert")
        .build()
        .unwrap();
    let video_sink = gst::ElementFactory::make("gtk4paintablesink")
        .name("video_sink")
        .build()
        .unwrap();

    let pipeline = gst::Pipeline::with_name("test-pipeline");

    let effect_bin = gst::Bin::with_name("effect_bin");
    effect_bin
        .add_many([&equalizer, &band_pass, &echo])
        .unwrap();
    gst::Element::link_many([&equalizer, &band_pass, &echo]).unwrap();
    effect_bin
        .add_pad(&gst::GhostPad::with_target(&equalizer.static_pad("sink").unwrap()).unwrap())
        .unwrap();
    effect_bin
        .add_pad(&gst::GhostPad::with_target(&echo.static_pad("src").unwrap()).unwrap())
        .unwrap();

    pipeline
        .add_many([
            &audio_source,
            &audio_amplify,
            effect_bin.upcast_ref::<gst::Element>(),
            &audio_convert,
            &tee,
            &audio_queue,
            &audio_convert_2,
            &audio_resample,
            &audio_sink,
            &video_queue,
            &visual,
            &video_convert,
            &video_sink,
        ])
        .unwrap();

    gst::Element::link_many([
        &audio_source,
        &audio_amplify,
        effect_bin.upcast_ref::<gst::Element>(),
        &audio_convert,
        &tee,
    ])
    .unwrap();
    gst::Element::link_many([&audio_queue, &audio_convert_2, &audio_resample, &audio_sink])
        .unwrap();
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

    pipeline
}
