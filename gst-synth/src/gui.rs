use crate::gui::key::key;
use crate::types::{Command, Note};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Orientation, glib};
use gtk4 as gtk;

mod key;

pub fn draw_gui(command_tx: async_channel::Sender<Command>) -> glib::ExitCode {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First GTK Program")
            .default_width(350)
            .default_height(200)
            .build();

        let keyboard = GtkBox::new(Orientation::Horizontal, 12);
        keyboard.set_margin_top(24);
        keyboard.set_margin_bottom(24);
        keyboard.set_margin_start(24);
        keyboard.set_margin_end(24);

        keyboard.append(&key(command_tx.clone(), Note::C, "C"));
        keyboard.append(&key(command_tx.clone(), Note::D, "D"));
        keyboard.append(&key(command_tx.clone(), Note::E, "E"));
        keyboard.append(&key(command_tx.clone(), Note::F, "F"));
        keyboard.append(&key(command_tx.clone(), Note::G, "G"));
        keyboard.append(&key(command_tx.clone(), Note::A, "A"));
        keyboard.append(&key(command_tx.clone(), Note::B, "B"));

        window.set_child(Some(&keyboard));

        window.present();
    });

    application.run()
}
