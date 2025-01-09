use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
use glib::BoolError;

pub fn create_and_run_app() -> Result<(), BoolError> {
    let app = Application::new(
        Some("com.example.gtk-rs"),
        Default::default(),
    );

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Postgres GUI");
        window.set_default_size(350, 70);

        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Button clicked!");
        });

        window.add(&button);
        window.show_all();
    });

    app.run();
    Ok(())
}