use crate::types::{Command, Note, WaveForm};
use gtk4::{self as gtk, gdk, glib, prelude::*};

pub fn attach_keyboard_handler(
    window: &gtk::ApplicationWindow,
    command_tx: async_channel::Sender<Command>,
) {
    let key_controller = gtk::EventControllerKey::new();
    key_controller.connect_key_pressed(move |_, key, _, _| {
        let command = match key {
            gdk::Key::q => Command::Quit,

            gdk::Key::a => Command::ChangeNote(Note::C),
            gdk::Key::w => Command::ChangeNote(Note::CSharp),
            gdk::Key::s => Command::ChangeNote(Note::D),
            gdk::Key::e => Command::ChangeNote(Note::DSharp),
            gdk::Key::d => Command::ChangeNote(Note::E),
            gdk::Key::f => Command::ChangeNote(Note::F),
            gdk::Key::t => Command::ChangeNote(Note::FSharp),
            gdk::Key::g => Command::ChangeNote(Note::G),
            gdk::Key::z | gdk::Key::y => Command::ChangeNote(Note::GSharp),
            gdk::Key::h => Command::ChangeNote(Note::A),
            gdk::Key::u => Command::ChangeNote(Note::ASharp),
            gdk::Key::j => Command::ChangeNote(Note::B),

            gdk::Key::v => Command::ChangeWaveForm(WaveForm::Sine),
            gdk::Key::b => Command::ChangeWaveForm(WaveForm::Square),
            gdk::Key::n => Command::ChangeWaveForm(WaveForm::Saw),
            gdk::Key::m => Command::ChangeWaveForm(WaveForm::Triangle),

            gdk::Key::_1 => Command::ChangeOctave(1),
            gdk::Key::_2 => Command::ChangeOctave(2),
            gdk::Key::_3 => Command::ChangeOctave(3),
            gdk::Key::_4 => Command::ChangeOctave(4),
            gdk::Key::_5 => Command::ChangeOctave(5),
            gdk::Key::_6 => Command::ChangeOctave(6),
            gdk::Key::_7 => Command::ChangeOctave(7),

            _ => return glib::Propagation::Proceed,
        };
        let _ = command_tx.send_blocking(command);
        glib::Propagation::Stop
    });
    window.add_controller(key_controller);
}
