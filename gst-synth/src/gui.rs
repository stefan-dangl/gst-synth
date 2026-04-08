use crate::gui::key::{Color, key, placeholder_key};
use crate::gui::style::style;
use crate::types::{Command, Note};
use gtk::prelude::*;
use gtk::{Align, Application, ApplicationWindow, Box as GtkBox, Orientation, Overlay, glib};
use gtk4 as gtk;

mod key;
mod style;

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

        let overlay = Overlay::new();
        overlay.set_hexpand(true);
        overlay.set_vexpand(true);

        let tones = GtkBox::new(Orientation::Horizontal, 12);
        tones.set_halign(Align::Center);
        tones.set_valign(Align::End);
        tones.set_margin_bottom(24);
        tones.set_margin_start(24);
        tones.set_margin_end(24);

        tones.append(&key(command_tx.clone(), Note::C, "C", Color::White));
        tones.append(&key(command_tx.clone(), Note::D, "D", Color::White));
        tones.append(&key(command_tx.clone(), Note::E, "E", Color::White));
        tones.append(&key(command_tx.clone(), Note::F, "F", Color::White));
        tones.append(&key(command_tx.clone(), Note::G, "G", Color::White));
        tones.append(&key(command_tx.clone(), Note::A, "A", Color::White));
        tones.append(&key(command_tx.clone(), Note::B, "B", Color::White));

        overlay.set_child(Some(&tones));

        let semitones = GtkBox::new(Orientation::Horizontal, 12);
        semitones.set_halign(Align::Center);
        semitones.set_valign(Align::End);
        semitones.set_margin_bottom(88);
        semitones.set_margin_start(66);

        semitones.append(&key(command_tx.clone(), Note::CSharp, "C#", Color::Black));
        semitones.append(&key(command_tx.clone(), Note::DSharp, "D#", Color::Black));
        semitones.append(&placeholder_key());
        semitones.append(&key(command_tx.clone(), Note::FSharp, "F#", Color::Black));
        semitones.append(&key(command_tx.clone(), Note::GSharp, "G#", Color::Black));
        semitones.append(&key(command_tx.clone(), Note::ASharp, "A#", Color::Black));
        semitones.append(&placeholder_key());

        overlay.add_overlay(&semitones);
        window.set_child(Some(&overlay));

        let style = style();
        gtk::style_context_add_provider_for_display(
            &gtk::prelude::WidgetExt::display(&window),
            &style,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        window.present();
    });

    application.run()
}
