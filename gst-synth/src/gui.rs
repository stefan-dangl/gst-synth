use crate::gui::keys::keyboard;
use crate::gui::style::style;
use crate::gui::visualization::visualization;
use crate::gui::waveform_selection::waveform_selection;
use crate::types::Command;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Overlay, glib};
use gtk4 as gtk;

mod keys;
mod style;
mod visualization;
mod waveform_selection;

pub fn draw_gui(
    command_tx: async_channel::Sender<Command>,
    // video_sink: gst::Element,
) -> glib::ExitCode {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First GTK Program")
            .default_width(1600)
            .default_height(600)
            .build();

        let overlay = Overlay::new();
        overlay.set_hexpand(true);
        overlay.set_vexpand(true);

        waveform_selection(&overlay, command_tx.clone());
        // visualization(&overlay, video_sink);
        keyboard(&overlay, command_tx.clone());
        window.set_child(Some(&overlay));

        gtk::style_context_add_provider_for_display(
            &gtk::prelude::WidgetExt::display(&window),
            &style(),
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        window.present();
    });

    application.run()
}
