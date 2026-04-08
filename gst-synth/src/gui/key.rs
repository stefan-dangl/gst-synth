use crate::types::{Command, Note};
use gtk::GestureClick;
use gtk::prelude::*;
use gtk::{Box as GtkBox, Frame, Label, Orientation, glib};
use gtk4 as gtk;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

const NOTE_REPEAT_INTERVAL: Duration = Duration::from_millis(5);

pub enum Color {
    White,
    Black,
}

pub fn key(
    command_tx: async_channel::Sender<Command>,
    note: Note,
    label: &str,
    key_color: Color,
) -> Frame {
    let key = Frame::new(None);
    key.set_size_request(72, 72);

    let label = Label::new(Some(label));

    match key_color {
        Color::White => {
            key.add_css_class("white-key");
            label.add_css_class("white-key-label");
        }
        Color::Black => {
            key.add_css_class("black-key");
            label.add_css_class("black-key-label");
        }
    }

    key.set_child(Some(&label));

    let repeat_source = Rc::new(RefCell::new(None::<glib::SourceId>));

    let gesture = GestureClick::new();

    {
        let command_tx = command_tx.clone();
        let repeat_source = repeat_source.clone();
        let key = key.clone();

        gesture.connect_pressed(move |_, _, _, _| {
            println!("!!! BUTTON PRESSED");
            key.add_css_class("active");

            if repeat_source.borrow().is_some() {
                return;
            }

            let _ = command_tx.try_send(Command::ChangeNote(note));

            let command_tx = command_tx.clone();
            let source_id = glib::timeout_add_local(NOTE_REPEAT_INTERVAL, move || {
                let _ = command_tx.try_send(Command::ChangeNote(note));
                glib::ControlFlow::Continue
            });

            repeat_source.borrow_mut().replace(source_id);
        });
    }

    {
        let repeat_source = repeat_source.clone();
        let key = key.clone();
        gesture.connect_released(move |_, _, _, _| {
            println!("!!! BUTTON RELEASED");
            key.remove_css_class("active");

            if let Some(source_id) = repeat_source.borrow_mut().take() {
                source_id.remove();
            }
        });
    }

    {
        let repeat_source = repeat_source.clone();
        let key = key.clone();
        gesture.connect_stopped(move |_| {
            key.remove_css_class("active");

            if let Some(source_id) = repeat_source.borrow_mut().take() {
                source_id.remove();
            }
        });
    }

    key.add_controller(gesture);

    key
}

pub fn placeholder_key() -> GtkBox {
    let placeholder = GtkBox::new(Orientation::Horizontal, 0);
    placeholder.set_size_request(72, 72);
    placeholder
}
