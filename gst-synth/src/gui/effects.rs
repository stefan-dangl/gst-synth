use gst::prelude::*;
use gtk::prelude::*;
use gtk::{Box as GtkBox, DrawingArea, Frame, GestureDrag, Label, Orientation, Overlay};
use gtk4 as gtk;
use gtk4::Align;
use gtk4::cairo;
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;

const KNOB_SIZE: i32 = 72;
const KNOB_START: f64 = PI * 0.75; // lower-left  (~8 o'clock)
const KNOB_SWEEP: f64 = PI * 1.5; // 270° clockwise to lower-right (~4 o'clock)

fn draw_knob(cr: &cairo::Context, w: i32, h: i32, value: f64, min: f64, max: f64) {
    let cx = w as f64 / 2.0;
    let cy = h as f64 / 2.0;
    let r = cx.min(cy) - 3.0;
    let track_r = r - 8.0;

    let t = if max > min {
        ((value - min) / (max - min)).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let angle = KNOB_START + t * KNOB_SWEEP;

    // Background circle
    cr.set_source_rgb(0.15, 0.15, 0.15);
    cr.arc(cx, cy, r, 0.0, 2.0 * PI);
    cr.fill().unwrap();

    // Full-range track
    cr.set_source_rgb(0.3, 0.3, 0.3);
    cr.set_line_width(4.0);
    cr.set_line_cap(cairo::LineCap::Round);
    cr.arc(cx, cy, track_r, KNOB_START, KNOB_START + KNOB_SWEEP);
    cr.stroke().unwrap();

    // Active arc
    if t > 0.0 {
        cr.set_source_rgb(0.9, 0.5, 0.1);
        cr.arc(cx, cy, track_r, KNOB_START, angle);
        cr.stroke().unwrap();
    }

    // Indicator tick
    cr.set_source_rgb(1.0, 1.0, 1.0);
    cr.set_line_width(2.0);
    cr.move_to(
        cx + (track_r - 5.0) * angle.cos(),
        cy + (track_r - 5.0) * angle.sin(),
    );
    cr.line_to(
        cx + (track_r + 3.0) * angle.cos(),
        cy + (track_r + 3.0) * angle.sin(),
    );
    cr.stroke().unwrap();
}

fn knob(
    label_text: &str,
    min: f64,
    max: f64,
    initial: f64,
    setter: impl Fn(f64) + 'static,
    fmt: impl Fn(f64) -> String + 'static,
) -> GtkBox {
    let value = Rc::new(RefCell::new(initial));

    let container = GtkBox::new(Orientation::Vertical, 2);
    container.set_margin_start(4);
    container.set_margin_end(4);

    // Header: label and value close together, centered above the knob
    let header = GtkBox::new(Orientation::Horizontal, 4);
    header.set_halign(Align::Center);
    let lbl = Label::new(Some(label_text));
    lbl.add_css_class("knob-label");
    let val_lbl = Label::new(Some(&fmt(initial)));
    val_lbl.add_css_class("knob-value");
    header.append(&lbl);
    header.append(&val_lbl);

    // Drawing area
    let da = DrawingArea::new();
    da.set_size_request(KNOB_SIZE, KNOB_SIZE);
    {
        let value = value.clone();
        da.set_draw_func(move |_, cr, w, h| draw_knob(cr, w, h, *value.borrow(), min, max));
    }

    // Drag up = increase, drag down = decrease
    let gesture = GestureDrag::new();
    let drag_origin = Rc::new(RefCell::new(initial));
    {
        let drag_origin = drag_origin.clone();
        let value = value.clone();
        gesture.connect_drag_begin(move |_, _, _| {
            *drag_origin.borrow_mut() = *value.borrow();
        });
    }
    {
        let da = da.clone();
        let val_lbl = val_lbl.clone();
        gesture.connect_drag_update(move |_, _dx, dy| {
            let new_val = (*drag_origin.borrow() - dy / 200.0 * (max - min)).clamp(min, max);
            *value.borrow_mut() = new_val;
            setter(new_val);
            val_lbl.set_text(&fmt(new_val));
            da.queue_draw();
        });
    }
    da.add_controller(gesture);

    container.append(&header);
    container.append(&da);
    container
}

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
    let delay_init = (echo.property::<u64>("delay") / 1_000_000) as f64;
    let intensity_init = echo.property::<f32>("intensity") as f64;
    let feedback_init = echo.property::<f32>("feedback") as f64;

    let filter_knobs = vec![
        {
            let e = bp.clone();
            knob(
                "Lower",
                0.0,
                20000.0,
                lower_init,
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
                move |v| e.set_property("band2", v),
                |v| format!("{:.1}", v),
            )
        },
        {
            let e = bp.clone();
            knob(
                "Upper",
                0.0,
                20000.0,
                upper_init,
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
                delay_init,
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
                intensity_init,
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
                feedback_init,
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
