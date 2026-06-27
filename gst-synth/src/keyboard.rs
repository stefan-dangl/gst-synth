use crate::types::{Command, Note, WaveForm};
use gtk4::{self as gtk, gdk, glib, prelude::*};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

pub const REPEAT_INTERVAL: Duration = Duration::from_millis(5);

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
    command_tx: async_channel::Sender<Command>,
    key_frames: HashMap<Note, gtk::Frame>,
    waveform_frames: HashMap<WaveForm, gtk::Frame>,
    selected_waveform: Rc<RefCell<Option<gtk::Frame>>>,
    octave_label: gtk::Label,
    octave_rc: Rc<RefCell<usize>>,
) {
    let key_controller = gtk::EventControllerKey::new();
    let key_frames = Rc::new(key_frames);

    let active_key: Rc<RefCell<Option<gdk::Key>>> = Rc::new(RefCell::new(None));
    let active_note: Rc<RefCell<Option<Note>>> = Rc::new(RefCell::new(None));
    let repeat_source: Rc<RefCell<Option<glib::SourceId>>> = Rc::new(RefCell::new(None));

    {
        let command_tx = command_tx.clone();
        let active_key = active_key.clone();
        let active_note = active_note.clone();
        let repeat_source = repeat_source.clone();
        let key_frames = key_frames.clone();

        key_controller.connect_key_pressed(move |_, key, _, _| {
            // Suppress OS auto-repeat events — we manage our own repeat timer
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

            // Unhighlight previous note key, highlight the new one
            if let Some(prev) = active_note.borrow_mut().take() {
                if let Some(frame) = key_frames.get(&prev) {
                    frame.remove_css_class("active");
                }
            }
            if let Command::ChangeNote(note) = command {
                if let Some(frame) = key_frames.get(&note) {
                    frame.add_css_class("active");
                }
                *active_note.borrow_mut() = Some(note);
            }

            if let Command::ChangeWaveForm(wf) = command {
                if let Some(prev) = selected_waveform.borrow().as_ref() {
                    prev.remove_css_class("selected");
                }
                if let Some(frame) = waveform_frames.get(&wf) {
                    frame.add_css_class("selected");
                    *selected_waveform.borrow_mut() = Some(frame.clone());
                }
            }

            if let Command::ChangeOctave(n) = command {
                octave_label.set_text(&format!("{n}"));
                *octave_rc.borrow_mut() = n;
            }

            let _ = command_tx.try_send(command);

            let command_tx = command_tx.clone();
            let id = glib::timeout_add_local(REPEAT_INTERVAL, move || {
                let _ = command_tx.try_send(command);
                glib::ControlFlow::Continue
            });
            *repeat_source.borrow_mut() = Some(id);

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
                if let Some(note) = active_note.borrow_mut().take() {
                    if let Some(frame) = key_frames.get(&note) {
                        frame.remove_css_class("active");
                    }
                }
            }
        });
    }

    window.add_controller(key_controller);
}
