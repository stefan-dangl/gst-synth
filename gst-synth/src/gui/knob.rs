use gtk::prelude::*;
use gtk::{Box as GtkBox, DrawingArea, Label, Orientation};
use gtk4 as gtk;
use gtk4::Align;
use gtk4::cairo;
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;

pub const KNOB_SIZE: i32 = 72;
const KNOB_START: f64 = PI * 0.75;
const KNOB_SWEEP: f64 = PI * 1.5;

pub fn draw_knob(cr: &cairo::Context, w: i32, h: i32, value: f64, min: f64, max: f64) {
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

    cr.set_source_rgb(0.15, 0.15, 0.15);
    cr.arc(cx, cy, r, 0.0, 2.0 * PI);
    cr.fill().unwrap();

    cr.set_source_rgb(0.3, 0.3, 0.3);
    cr.set_line_width(4.0);
    cr.set_line_cap(cairo::LineCap::Round);
    cr.arc(cx, cy, track_r, KNOB_START, KNOB_START + KNOB_SWEEP);
    cr.stroke().unwrap();

    if t > 0.0 {
        cr.set_source_rgb(0.9, 0.5, 0.1);
        cr.arc(cx, cy, track_r, KNOB_START, angle);
        cr.stroke().unwrap();
    }

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

pub fn knob(
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

    let header = GtkBox::new(Orientation::Horizontal, 4);
    header.set_halign(Align::Center);
    let lbl = Label::new(Some(label_text));
    lbl.add_css_class("knob-label");
    let val_lbl = Label::new(Some(&fmt(initial)));
    val_lbl.add_css_class("knob-value");
    header.append(&lbl);
    header.append(&val_lbl);

    let da = DrawingArea::new();
    da.set_size_request(KNOB_SIZE, KNOB_SIZE);
    {
        let value = value.clone();
        da.set_draw_func(move |_, cr, w, h| draw_knob(cr, w, h, *value.borrow(), min, max));
    }

    let gesture = gtk::GestureDrag::new();
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
