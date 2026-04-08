use crate::types::{Command, Note, WaveForm};
use std::{io, thread, time};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub fn handle_keyboard(command_tx: async_channel::Sender<Command>) {
    let _stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().keys();

    loop {
        if let Some(Ok(input)) = stdin.next() {
            let command = match input {
                Key::Char('q') => Command::Quit,

                Key::Char('a') => Command::ChangeNote(Note::C),
                Key::Char('w') => Command::ChangeNote(Note::CSharp),
                Key::Char('s') => Command::ChangeNote(Note::D),
                Key::Char('e') => Command::ChangeNote(Note::DSharp),
                Key::Char('d') => Command::ChangeNote(Note::E),
                Key::Char('f') => Command::ChangeNote(Note::F),
                Key::Char('t') => Command::ChangeNote(Note::FSharp),
                Key::Char('g') => Command::ChangeNote(Note::G),
                Key::Char('z' | 'y') => Command::ChangeNote(Note::GSharp), // y to support german keyboards
                Key::Char('h') => Command::ChangeNote(Note::A),
                Key::Char('u') => Command::ChangeNote(Note::ASharp),
                Key::Char('j') => Command::ChangeNote(Note::B),

                Key::Char('v') => Command::ChangeWaveForm(WaveForm::Sine),
                Key::Char('b') => Command::ChangeWaveForm(WaveForm::Square),
                Key::Char('n') => Command::ChangeWaveForm(WaveForm::Saw),
                Key::Char('m') => Command::ChangeWaveForm(WaveForm::Triangle),

                Key::Char('1') => Command::ChangeOctave(1),
                Key::Char('2') => Command::ChangeOctave(2),
                Key::Char('3') => Command::ChangeOctave(3),
                Key::Char('4') => Command::ChangeOctave(4),
                Key::Char('5') => Command::ChangeOctave(5),
                Key::Char('6') => Command::ChangeOctave(6),
                Key::Char('7') => Command::ChangeOctave(7),
                _ => continue,
            };
            command_tx
                .send_blocking(command)
                .expect("failed to send data through channel");
        }

        thread::sleep(time::Duration::from_millis(5));
    }
}
