use crate::types::{DEFAULT_ECHO_DELAY, DEFAULT_ECHO_FEEDBACK, DEFAULT_ECHO_INTENSITY};

use super::knob::knob;
use gst::prelude::*;
use gtk::prelude::*;
use gtk::{Box as GtkBox, Frame, Orientation, Overlay};
use gtk4 as gtk;
use gtk4::Align;

fn section(title: &str, knobs: Vec<GtkBox>) -> Frame {
    let frame = Frame::new(Some(title));
    frame.add_css_class("effect-section");
    let row = GtkBox::new(Orientation::Horizontal, 4);
    row.set_margin_top(8);
    row.set_margin_bottom(8);
    row.set_margin_start(8);
    row.set_margin_end(8);
    for k in knobs {
        row.append(&k);
    }
    frame.set_child(Some(&row));
    frame
}

pub fn effects(overlay: &Overlay, effect_bin: gst::Element) {
    let bin = effect_bin.downcast::<gst::Bin>().unwrap();
    let eq = bin.by_name("audio_equalizer").unwrap();
    let bp = bin.by_name("audio_band_pass").unwrap();
    let echo = bin.by_name("audio_echo").unwrap();

    let lower_init = bp.property::<f32>("lower-frequency") as f64;
    let upper_init = bp.property::<f32>("upper-frequency") as f64;
    let band0_init = eq.property::<f64>("band0");
    let band1_init = eq.property::<f64>("band1");
    let band2_init = eq.property::<f64>("band2");

    let filter_knobs = vec![
        {
            let e = bp.clone();
            knob(
                "Lower [Hz]",
                20.0,
                20000.0,
                lower_init.max(20.0),
                true,
                move |v| e.set_property("lower-frequency", v as f32),
                |v| format!("{:.0}", v),
            )
        },
        {
            let e = eq.clone();
            knob(
                "100Hz",
                -24.0,
                12.0,
                band0_init,
                false,
                move |v| e.set_property("band0", v),
                |v| format!("{:.1}", v),
            )
        },
        {
            let e = eq.clone();
            knob(
                "1.1kHz",
                -24.0,
                12.0,
                band1_init,
                false,
                move |v| e.set_property("band1", v),
                |v| format!("{:.1}", v),
            )
        },
        {
            let e = eq.clone();
            knob(
                "11kHz",
                -24.0,
                12.0,
                band2_init,
                false,
                move |v| e.set_property("band2", v),
                |v| format!("{:.1}", v),
            )
        },
        {
            let e = bp.clone();
            knob(
                "Upper [Hz]",
                20.0,
                20000.0,
                upper_init.max(20.0),
                true,
                move |v| e.set_property("upper-frequency", v as f32),
                |v| format!("{:.0}", v),
            )
        },
    ];

    let echo_knobs = vec![
        {
            let e = echo.clone();
            knob(
                "Delay [ms]",
                0.1,
                3000.0,
                DEFAULT_ECHO_DELAY,
                false,
                move |v| e.set_property("delay", (v * 1_000_000.0) as u64),
                |v| format!("{:.0}", v),
            )
        },
        {
            let e = echo.clone();
            knob(
                "Intensity",
                0.0,
                1.0,
                DEFAULT_ECHO_INTENSITY,
                false,
                move |v| e.set_property("intensity", v as f32),
                |v| format!("{:.2}", v),
            )
        },
        {
            let e = echo.clone();
            knob(
                "Feedback",
                0.0,
                1.0,
                DEFAULT_ECHO_FEEDBACK,
                false,
                move |v| e.set_property("feedback", v as f32),
                |v| format!("{:.2}", v),
            )
        },
    ];

    let panel = GtkBox::new(Orientation::Vertical, 8);
    panel.set_halign(Align::Center);
    panel.set_valign(Align::Start);
    panel.set_margin_top(24);
    panel.set_margin_start(8);
    panel.set_margin_end(8);
    panel.append(&section("Filter", filter_knobs));
    panel.append(&section("Echo", echo_knobs));

    overlay.add_overlay(&panel);
}
