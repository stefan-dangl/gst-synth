use crate::config::NOTE_REPEAT_INTERVAL;
use crate::types::{Command, Note, UiEvent, WaveForm};
use gtk4::{self as gtk, gdk, glib, prelude::*};
use std::cell::RefCell;
use std::rc::Rc;

fn key_to_command(key: gdk::Key) -> Option<Command> {
    match key {
        gdk::Key::q => Some(Command::Quit),

        gdk::Key::a => Some(Command::ChangeNote(Note::C)),
        gdk::Key::w => Some(Command::ChangeNote(Note::CSharp)),
        gdk::Key::s => Some(Command::ChangeNote(Note::D)),
        gdk::Key::e => Some(Command::ChangeNote(Note::DSharp)),
        gdk::Key::d => Some(Command::ChangeNote(Note::E)),
        gdk::Key::f => Some(Command::ChangeNote(Note::F)),
        gdk::Key::t => Some(Command::ChangeNote(Note::FSharp)),
        gdk::Key::g => Some(Command::ChangeNote(Note::G)),
        gdk::Key::z | gdk::Key::y => Some(Command::ChangeNote(Note::GSharp)),
        gdk::Key::h => Some(Command::ChangeNote(Note::A)),
        gdk::Key::u => Some(Command::ChangeNote(Note::ASharp)),
        gdk::Key::j => Some(Command::ChangeNote(Note::B)),

        gdk::Key::v => Some(Command::ChangeWaveForm(WaveForm::Saw)),
        gdk::Key::b => Some(Command::ChangeWaveForm(WaveForm::Square)),
        gdk::Key::n => Some(Command::ChangeWaveForm(WaveForm::Triangle)),
        gdk::Key::m => Some(Command::ChangeWaveForm(WaveForm::Sine)),

        gdk::Key::_1 => Some(Command::ChangeOctave(1)),
        gdk::Key::_2 => Some(Command::ChangeOctave(2)),
        gdk::Key::_3 => Some(Command::ChangeOctave(3)),
        gdk::Key::_4 => Some(Command::ChangeOctave(4)),
        gdk::Key::_5 => Some(Command::ChangeOctave(5)),
        gdk::Key::_6 => Some(Command::ChangeOctave(6)),
        gdk::Key::_7 => Some(Command::ChangeOctave(7)),

        _ => None,
    }
}

pub fn attach_keyboard_handler(
    window: &gtk::ApplicationWindow,
    command_tx: &async_channel::Sender<Command>,
    ui_tx: async_channel::Sender<UiEvent>,
) {
    let key_controller = gtk::EventControllerKey::new();

    let active_key: Rc<RefCell<Option<gdk::Key>>> = Rc::new(RefCell::new(None));
    let repeat_source: Rc<RefCell<Option<glib::SourceId>>> = Rc::new(RefCell::new(None));

    {
        let command_tx = command_tx.clone();
        let active_key = active_key.clone();
        let repeat_source = repeat_source.clone();
        let ui_tx = ui_tx.clone();

        key_controller.connect_key_pressed(move |_, key, _, _| {
            if active_key.borrow().as_ref() == Some(&key) {
                return glib::Propagation::Stop;
            }

            let Some(command) = key_to_command(key) else {
                return glib::Propagation::Proceed;
            };

            // A different key was pressed while another was held: cancel the old timer
            if let Some(id) = repeat_source.borrow_mut().take() {
                id.remove();
            }
            *active_key.borrow_mut() = Some(key);

            let ui_event = match command {
                Command::ChangeNote(note) => Some(UiEvent::Note(Some(note))),
                Command::ChangeWaveForm(wf) => Some(UiEvent::WaveForm(wf)),
                Command::ChangeOctave(n) => Some(UiEvent::Octave(n)),
                _ => None,
            };
            if let Some(ui_event) = ui_event
                && let Err(err) = ui_tx.try_send(ui_event)
            {
                eprintln!("Failed to send ui event: {err:?}");
            }

            if let Err(err) = command_tx.try_send(command) {
                eprintln!("Failed to send command: {err:?}");
            }

            let command_tx = command_tx.clone();
            let command_repeater = glib::timeout_add_local(NOTE_REPEAT_INTERVAL, move || {
                if let Err(err) = command_tx.try_send(command) {
                    eprintln!("Failed to repeat keyboard command: {err:?}");
                }
                glib::ControlFlow::Continue
            });
            *repeat_source.borrow_mut() = Some(command_repeater);

            glib::Propagation::Stop
        });
    }

    {
        key_controller.connect_key_released(move |_, key, _, _| {
            if active_key.borrow().as_ref() == Some(&key) {
                *active_key.borrow_mut() = None;
                if let Some(id) = repeat_source.borrow_mut().take() {
                    id.remove();
                }
                let _ = ui_tx.try_send(UiEvent::Note(None));
            }
        });
    }

    window.add_controller(key_controller);
}
