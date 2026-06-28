use crate::gui::{
    effects::effects, keys::keyboard, octave_selection::octave_selection, style::style,
    visualization::visualization, waveform_selection::waveform_selection,
};
use crate::keyboard::attach_keyboard_handler;
use crate::types::{Command, DEFAULT_HEIGHT, DEFAULT_WIDTH, GUI_TITLE};
use gtk4::{
    self as gtk, Align, Application, ApplicationWindow, Box as GtkBox, Orientation, Overlay,
    SizeGroup, SizeGroupMode, glib, prelude::*,
};

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
        .application_id("io.github.gst_synth")
        .build();

    application.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title(GUI_TITLE)
            .default_width(DEFAULT_WIDTH)
            .default_height(DEFAULT_HEIGHT)
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
        let (waveform_selection, waveforms, selected_waveform) = waveform_selection(&command_tx);
        let visualization = visualization(&video_sink);
        width_group.add_widget(&waveform_selection);
        width_group.add_widget(&visualization);

        let spacer = GtkBox::new(Orientation::Horizontal, 0);
        spacer.set_hexpand(true);
        top_row.append(&waveform_selection);
        top_row.append(&spacer);
        top_row.append(&visualization);
        overlay.add_overlay(&top_row);

        effects(&overlay, effect_bin.clone());
        // TODO_SD: Command -> new label
        let (octave_label, octave_rc) = octave_selection(&overlay, &command_tx);
        let key_map = keyboard(&overlay, &command_tx);

        window.set_child(Some(&overlay));

        attach_keyboard_handler(
            &window,
            &command_tx,
            key_map,
            waveforms,
            selected_waveform,
            octave_label, // TODO_SD: Not needed
            octave_rc,
        );

        gtk::style_context_add_provider_for_display(
            &WidgetExt::display(&window),
            &style(),
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        window.present();
    });

    application.run()
}
