use crate::types::WaveForm;
use crate::types::{Command, Note};
use gtk::GestureClick;
use gtk::prelude::*;
use gtk::{Box as GtkBox, Frame, Label, Orientation, Overlay};
use gtk4 as gtk;
use gtk4::Align;

fn button(command_tx: async_channel::Sender<Command>, label: &str, waveform: WaveForm) -> Frame {
    let key = Frame::new(None);
    key.set_size_request(72, 72);

    let label = Label::new(Some(label));
    key.add_css_class("black-key");
    label.add_css_class("black-key-label");

    key.set_child(Some(&label));

    let gesture = GestureClick::new();

    {
        let command_tx = command_tx.clone();
        let key = key.clone();

        gesture.connect_pressed(move |_, _, _, _| {
            println!("!!! WAVEFORM BUTTON PRESSED");
            key.add_css_class("active"); // TODO_SD: Defined?
            let _ = command_tx.try_send(Command::ChangeWaveForm(waveform));
        });
    }

    key.add_controller(gesture);

    key
}

pub fn waveform_selection(overlay: &Overlay, command_tx: async_channel::Sender<Command>) {
    let waveforms = GtkBox::new(Orientation::Horizontal, 12);
    waveforms.set_halign(Align::Start);
    waveforms.set_valign(Align::Start);
    waveforms.set_margin_top(24);
    waveforms.set_margin_start(24);
    waveforms.set_margin_end(24);

    waveforms.append(&button(command_tx.clone(), "Sn", WaveForm::Sine));
    waveforms.append(&button(command_tx.clone(), "Sq", WaveForm::Square));
    waveforms.append(&button(command_tx.clone(), "Sw", WaveForm::Saw));
    waveforms.append(&button(command_tx.clone(), "Tr", WaveForm::Triangle));

    overlay.add_overlay(&waveforms);
}
