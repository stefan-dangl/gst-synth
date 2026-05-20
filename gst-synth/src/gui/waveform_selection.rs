use super::knob::knob;
use crate::types::{ATTACK_TIME_DEFAULT, Command, RELEASE_TIME_DEFAULT, Setting, WaveForm};
use gtk::DrawingArea;
use gtk::GestureClick;
use gtk::prelude::*;
use gtk::{Box as GtkBox, Frame, Orientation};
use gtk4 as gtk;
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;
use std::time::Duration;

fn waveform_icon(waveform: WaveForm) -> DrawingArea {
    let area = DrawingArea::new();
    area.set_size_request(56, 36);
    area.set_draw_func(move |_, cr, width, height| {
        let w = width as f64;
        let h = height as f64;
        let cy = h / 2.0;
        let amp = h * 0.38;
        let x0 = w * 0.08;
        let x1 = w - x0;

        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.set_line_width(1.5);
        cr.set_line_cap(gtk::cairo::LineCap::Round);
        cr.set_line_join(gtk::cairo::LineJoin::Round);

        match waveform {
            WaveForm::Sine => {
                for i in 0..=100u32 {
                    let t = i as f64 / 100.0;
                    let x = x0 + t * (x1 - x0);
                    let y = cy - amp * (2.0 * PI * t).sin();
                    if i == 0 {
                        cr.move_to(x, y);
                    } else {
                        cr.line_to(x, y);
                    }
                }
            }
            WaveForm::Square => {
                let mid = (x0 + x1) / 2.0;
                cr.move_to(x0, cy - amp);
                cr.line_to(mid, cy - amp);
                cr.line_to(mid, cy + amp);
                cr.line_to(x1, cy + amp);
            }
            WaveForm::Saw => {
                cr.move_to(x0, cy + amp);
                cr.line_to(x1, cy - amp);
                cr.line_to(x1, cy + amp);
            }
            WaveForm::Triangle => {
                let span = x1 - x0;
                cr.move_to(x0, cy);
                cr.line_to(x0 + span * 0.25, cy - amp);
                cr.line_to(x0 + span * 0.75, cy + amp);
                cr.line_to(x1, cy);
            }
        }

        let _ = cr.stroke();
    });
    area
}

fn waveform_button(
    command_tx: async_channel::Sender<Command>,
    waveform: WaveForm,
    selected: Rc<RefCell<Option<Frame>>>,
) -> Frame {
    let frame = Frame::new(None);
    frame.set_size_request(72, 72);
    frame.add_css_class("black-key");
    frame.set_child(Some(&waveform_icon(waveform)));

    let gesture = GestureClick::new();
    {
        let frame = frame.clone();
        gesture.connect_pressed(move |_, _, _, _| {
            if let Some(prev) = selected.borrow().as_ref() {
                prev.remove_css_class("selected");
            }
            frame.add_css_class("selected");
            *selected.borrow_mut() = Some(frame.clone());
            let _ = command_tx.try_send(Command::ChangeWaveForm(waveform));
        });
    }
    frame.add_controller(gesture);
    frame
}

pub fn waveform_selection(command_tx: async_channel::Sender<Command>) -> Frame {
    let frame = Frame::new(Some("Waveform"));
    frame.add_css_class("effect-section");

    let inner = GtkBox::new(Orientation::Vertical, 8);
    inner.set_margin_top(8);
    inner.set_margin_bottom(8);
    inner.set_margin_start(8);
    inner.set_margin_end(8);

    let selected: Rc<RefCell<Option<Frame>>> = Rc::new(RefCell::new(None));

    let waveforms = GtkBox::new(Orientation::Horizontal, 12);
    waveforms.set_halign(gtk4::Align::Center);
    for wf in [
        WaveForm::Saw,
        WaveForm::Square,
        WaveForm::Triangle,
        WaveForm::Sine,
    ] {
        let btn = waveform_button(command_tx.clone(), wf, selected.clone());
        if wf == WaveForm::default() {
            btn.add_css_class("selected");
            *selected.borrow_mut() = Some(btn.clone());
        }
        waveforms.append(&btn);
    }
    inner.append(&waveforms);

    let envelope = GtkBox::new(Orientation::Horizontal, 12);
    envelope.set_halign(gtk4::Align::Center);
    {
        let tx = command_tx.clone();
        envelope.append(&knob(
            "Attack [ms]",
            0.0,
            2000.0,
            ATTACK_TIME_DEFAULT.as_millis() as f64,
            false,
            move |v| {
                let _ = tx.try_send(Command::ChangeSetting(Setting::AttackTime(
                    Duration::from_millis(v as u64),
                )));
            },
            |v| format!("{:.0}", v),
        ));
    }
    {
        let tx = command_tx.clone();
        envelope.append(&knob(
            "Release [ms]",
            0.0,
            5000.0,
            RELEASE_TIME_DEFAULT.as_millis() as f64,
            false,
            move |v| {
                let _ = tx.try_send(Command::ChangeSetting(Setting::ReleaseTime(
                    Duration::from_millis(v as u64),
                )));
            },
            |v| format!("{:.0}", v),
        ));
    }
    inner.append(&envelope);

    frame.set_child(Some(&inner));
    frame
}
