use crate::types::{Command, WaveForm};
use gtk::DrawingArea;
use gtk::GestureClick;
use gtk::prelude::*;
use gtk::{Box as GtkBox, Frame, Orientation, Overlay};
use gtk4 as gtk;
use gtk4::Align;
use std::f64::consts::PI;

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

fn button(command_tx: async_channel::Sender<Command>, waveform: WaveForm) -> Frame {
    let frame = Frame::new(None);
    frame.set_size_request(72, 72);
    frame.add_css_class("black-key");
    frame.set_child(Some(&waveform_icon(waveform)));

    let gesture = GestureClick::new();
    {
        let command_tx = command_tx.clone();
        let frame = frame.clone();
        gesture.connect_pressed(move |_, _, _, _| {
            frame.add_css_class("active");
            let _ = command_tx.try_send(Command::ChangeWaveForm(waveform));
        });
    }
    frame.add_controller(gesture);
    frame
}

pub fn waveform_selection(overlay: &Overlay, command_tx: async_channel::Sender<Command>) {
    let waveforms = GtkBox::new(Orientation::Horizontal, 12);
    waveforms.set_halign(Align::Start);
    waveforms.set_valign(Align::Start);
    waveforms.set_margin_top(24);
    waveforms.set_margin_start(24);
    waveforms.set_margin_end(24);

    waveforms.append(&button(command_tx.clone(), WaveForm::Sine));
    waveforms.append(&button(command_tx.clone(), WaveForm::Square));
    waveforms.append(&button(command_tx.clone(), WaveForm::Saw));
    waveforms.append(&button(command_tx.clone(), WaveForm::Triangle));

    overlay.add_overlay(&waveforms);
}
