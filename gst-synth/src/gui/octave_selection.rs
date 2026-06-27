use crate::types::Command;
use crate::types::OCTAVE_DEFAULT;
use crate::types::OCTAVE_MAX;
use crate::types::OCTAVE_MIN;
use gtk::GestureClick;
use gtk::prelude::*;
use gtk::{Frame, Grid, Label, Overlay};
use gtk4 as gtk;
use gtk4::Align;
use std::cell::RefCell;
use std::rc::Rc;

pub fn octave_selection(
    overlay: &Overlay,
    command_tx: async_channel::Sender<Command>,
) -> (Label, Rc<RefCell<usize>>) {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    container.set_halign(Align::Start);
    container.set_valign(Align::End);
    container.set_margin_bottom(24);
    container.set_margin_start(24);

    let octave = Rc::new(RefCell::new(OCTAVE_DEFAULT));
    let octave_label = Label::new(Some("Octave"));
    octave_label.add_css_class("black-key-label");

    let value = Label::new(Some(&format!("{OCTAVE_DEFAULT}")));
    value.add_css_class("octave-value");
    value.set_valign(Align::Center);

    let up_btn = arrow_button("▲");
    let down_btn = arrow_button("▼");

    {
        let command_tx = command_tx.clone();
        let octave = octave.clone();
        let value = value.clone();
        let gesture = GestureClick::new();
        gesture.connect_pressed(move |_, _, _, _| {
            let mut val = octave.borrow_mut();
            if *val < OCTAVE_MAX {
                *val += 1;
                value.set_text(&format!("{}", *val));
                let _ = command_tx.try_send(Command::ChangeOctave(*val));
            }
        });
        up_btn.add_controller(gesture);
    }

    {
        let command_tx = command_tx.clone();
        let octave = octave.clone();
        let value = value.clone();
        let gesture = GestureClick::new();
        gesture.connect_pressed(move |_, _, _, _| {
            let mut val = octave.borrow_mut();
            if *val > OCTAVE_MIN {
                *val -= 1;
                value.set_text(&format!("{}", *val));
                let _ = command_tx.try_send(Command::ChangeOctave(*val));
            }
        });
        down_btn.add_controller(gesture);
    }

    octave_label.set_halign(Align::Center);

    let grid = Grid::new();
    grid.set_row_spacing(8);
    grid.set_column_spacing(12);
    grid.attach(&octave_label, 0, 0, 1, 1);
    grid.attach(&up_btn, 0, 1, 1, 1);
    grid.attach(&down_btn, 0, 2, 1, 1);
    grid.attach(&value, 1, 1, 1, 2);

    container.append(&grid);

    overlay.add_overlay(&container);
    (value, octave)
}

fn arrow_button(symbol: &str) -> Frame {
    let frame = Frame::new(None);
    frame.set_size_request(72, 36);
    frame.add_css_class("black-key");
    let label = Label::new(Some(symbol));
    label.add_css_class("black-key-label");
    frame.set_child(Some(&label));
    frame
}
