use crate::types::{Command, Note};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, glib};
use gtk4 as gtk;

pub fn draw_gui(command_tx: async_channel::Sender<Command>) -> glib::ExitCode {
    println!("!!! draw gui");

    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First GTK Program")
            .default_width(350)
            .default_height(70)
            .build();

        let button = Button::with_label("Click me!");
        let command_tx_clone = command_tx.clone();
        button.connect_clicked(move |_| {
            eprintln!("Clicked!");
            command_tx_clone
                .send_blocking(Command::ChangeNote(Note::A))
                .expect("failed to send data through channel");
        });
        window.set_child(Some(&button));

        window.present();
    });

    println!("!!! application run");
    application.run()
}
