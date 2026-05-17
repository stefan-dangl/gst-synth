use crate::gui::effects::effects;
use crate::gui::keys::keyboard;
use crate::gui::octave_selection::octave_selection;
use crate::gui::style::style;
use crate::gui::visualization::visualization;
use crate::gui::waveform_selection::waveform_selection;
use crate::keyboard::attach_keyboard_handler;
use crate::types::Command;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Orientation, Overlay, SizeGroup, SizeGroupMode, glib};
use gtk4 as gtk;
use gtk4::Align;

mod effects;
mod keys;
mod knob;
mod octave_selection;
mod style;
mod visualization;
mod waveform_selection;

pub fn draw_gui(
    command_tx: async_channel::Sender<Command>,
    video_sink: gst::Element,
    effect_bin: gst::Element,
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

        let top_row = GtkBox::new(Orientation::Horizontal, 16);
        top_row.set_halign(Align::Fill);
        top_row.set_valign(Align::Start);
        top_row.set_hexpand(true);
        top_row.set_margin_top(24);
        top_row.set_margin_start(24);
        top_row.set_margin_end(24);

        let width_group = SizeGroup::new(SizeGroupMode::Horizontal);

        let wf_frame = waveform_selection(command_tx.clone());
        width_group.add_widget(&wf_frame);
        top_row.append(&wf_frame);

        let spacer = GtkBox::new(Orientation::Horizontal, 0);
        spacer.set_hexpand(true);
        top_row.append(&spacer);

        let vis = visualization(video_sink.clone());
        width_group.add_widget(&vis);
        top_row.append(&vis);

        overlay.add_overlay(&top_row);

        effects(&overlay, effect_bin.clone());
        octave_selection(&overlay, command_tx.clone());
        keyboard(&overlay, command_tx.clone());
        window.set_child(Some(&overlay));

        attach_keyboard_handler(&window, command_tx.clone());

        gtk::style_context_add_provider_for_display(
            &gtk::prelude::WidgetExt::display(&window),
            &style(),
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        window.present();
    });

    application.run()
}
