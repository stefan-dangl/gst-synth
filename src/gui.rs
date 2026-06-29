use crate::gui::{
    effects::effects, keyboard::attach_keyboard_handler, keys::keyboard,
    octave_selection::octave_selection, style::style, visualization::visualization,
    waveform_selection::waveform_selection,
};
use crate::config::{DEFAULT_HEIGHT, DEFAULT_WIDTH, GUI_TITLE};
use crate::types::{Command, Note, UiEvent, WaveForm};
use gtk4::{
    self as gtk, Align, Application, ApplicationWindow, Box as GtkBox, Orientation, Overlay,
    SizeGroup, SizeGroupMode, glib, prelude::*,
};

mod effects;
mod keyboard;
mod keys;
mod knob;
mod octave_selection;
mod style;
mod visualization;
mod waveform_selection;

fn spawn_ui_dispatcher(
    ui_rx: async_channel::Receiver<UiEvent>,
    update_note: impl Fn(Option<Note>) + 'static,
    update_octave: impl Fn(i32) + 'static,
    update_waveform: impl Fn(WaveForm) + 'static,
) {
    glib::MainContext::default().spawn_local(async move {
        while let Ok(event) = ui_rx.recv().await {
            match event {
                UiEvent::NoteChanged(note) => update_note(note),
                UiEvent::OctaveChanged(n) => update_octave(n),
                UiEvent::WaveFormChanged(wf) => update_waveform(wf),
            }
        }
    });
}

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
        let (waveform_widget, update_waveform) = waveform_selection(&command_tx);
        let visualization = visualization(&video_sink);
        width_group.add_widget(&waveform_widget);
        width_group.add_widget(&visualization);

        let spacer = GtkBox::new(Orientation::Horizontal, 0);
        spacer.set_hexpand(true);
        top_row.append(&waveform_widget);
        top_row.append(&spacer);
        top_row.append(&visualization);
        overlay.add_overlay(&top_row);

        effects(&overlay, effect_bin.clone());
        let update_octave = octave_selection(&overlay, &command_tx);
        let update_note = keyboard(&overlay, &command_tx);

        window.set_child(Some(&overlay));

        let (ui_tx, ui_rx) = async_channel::bounded::<UiEvent>(24);
        spawn_ui_dispatcher(ui_rx, update_note, update_octave, update_waveform);

        attach_keyboard_handler(&window, &command_tx, ui_tx);

        gtk::style_context_add_provider_for_display(
            &WidgetExt::display(&window),
            &style(),
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        window.present();
    });

    application.run()
}
