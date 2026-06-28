use super::knob::knob;
use crate::types::{ATTACK_TIME_DEFAULT, Command, RELEASE_TIME_DEFAULT, Setting, WaveForm};
use gtk4::{self as gtk, Box as GtkBox, DrawingArea, Frame, GestureClick, Orientation, prelude::*};
use std::{cell::RefCell, collections::HashMap, f64::consts::PI, rc::Rc, time::Duration};

fn waveform_icon(waveform: WaveForm) -> DrawingArea {
    let area = DrawingArea::new();
    area.set_size_request(56, 36);
    area.set_draw_func(move |_, cr, width, height| {
        let width_f = f64::from(width);
        let height_f = f64::from(height);
        let cy = height_f / 2.0;
        let amp = height_f * 0.38;
        let x0 = width_f * 0.08;
        let x1 = width_f - x0;

        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.set_line_width(1.5);
        cr.set_line_cap(gtk::cairo::LineCap::Round);
        cr.set_line_join(gtk::cairo::LineJoin::Round);

        match waveform {
            WaveForm::Sine => {
                for i in 0..=100u32 {
                    let frac = f64::from(i) / 100.0;
                    let px = x0 + frac * (x1 - x0);
                    let py = cy - amp * (2.0 * PI * frac).sin();
                    if i == 0 {
                        cr.move_to(px, py);
                    } else {
                        cr.line_to(px, py);
                    }
                }
            }
            WaveForm::Square => {
                let mid = f64::midpoint(x0, x1);
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

pub fn waveform_selection(
    command_tx: &async_channel::Sender<Command>,
) -> (Frame, HashMap<WaveForm, Frame>, Rc<RefCell<Option<Frame>>>) {
    let frame = Frame::new(Some("Waveform"));
    frame.add_css_class("effect-section");

    let inner = GtkBox::new(Orientation::Vertical, 8);
    inner.set_margin_top(8);
    inner.set_margin_bottom(8);
    inner.set_margin_start(8);
    inner.set_margin_end(8);

    let selected: Rc<RefCell<Option<Frame>>> = Rc::new(RefCell::new(None));
    let mut waveform_frames: HashMap<WaveForm, Frame> = HashMap::new();

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
        waveform_frames.insert(wf, btn.clone());
        waveforms.append(&btn);
    }
    inner.append(&waveforms);

    // TODO_SD: READ FROM COMMAND RX?
    let envelope = GtkBox::new(Orientation::Horizontal, 12);
    envelope.set_halign(gtk4::Align::Center);
    {
        let tx = command_tx.clone();
        envelope.append(&knob(
            "Attack [ms]",
            0.0,
            2000.0,
            ATTACK_TIME_DEFAULT.as_secs_f64() * 1000.0,
            false,
            move |v| {
                let _ = tx.try_send(Command::ChangeSetting(Setting::AttackTime(
                    Duration::from_secs_f64(v / 1000.0),
                )));
            },
            |v| format!("{v:.0}"),
        ));
    }
    {
        let tx = command_tx.clone();
        envelope.append(&knob(
            "Release [ms]",
            0.0,
            5000.0,
            RELEASE_TIME_DEFAULT.as_secs_f64() * 1000.0,
            false,
            move |v| {
                let _ = tx.try_send(Command::ChangeSetting(Setting::ReleaseTime(
                    Duration::from_secs_f64(v / 1000.0),
                )));
            },
            |v| format!("{v:.0}"),
        ));
    }
    inner.append(&envelope);

    frame.set_child(Some(&inner));
    (frame, waveform_frames, selected)
}
