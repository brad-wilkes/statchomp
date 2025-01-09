use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, ListBox, ListBoxRow, Label};
use glib::BoolError;
use crate::db::{team};
use std::sync::Arc;

pub async fn create_and_run_app() -> Result<(), BoolError> {
    let app = Application::new(
        Some("com.example.gtk-rs"),
        Default::default(),
    );

    app.connect_startup(|_| {
    });

    app.connect_activate(|app| {
        let app_clone = app.clone();
        glib::MainContext::default().spawn_local(async move {
            let window = ApplicationWindow::new(&app_clone);
            window.set_title("Postgres GUI");
            window.set_default_size(350, 200);

            let list_box = ListBox::new();
            window.add(&list_box);

            let list_box_clone = list_box.clone();
            let client = Arc::new(crate::db::connect_to_db().await.unwrap().0);
            let client_clone = Arc::clone(&client);

            glib::MainContext::default().spawn_local(async move {
                let teams = team::get_teams(&client_clone).await.unwrap();
                for team in teams {
                    let row = ListBoxRow::new();
                    let label = Label::new(Some(&format!("{}: {} - {}", team.id, team.college, team.mascot)));
                    row.add(&label);
                    list_box_clone.add(&row);
                }
                list_box_clone.show_all();
            });

            let button = Button::with_label("Add Team");
            let client_clone = Arc::clone(&client);
            button.connect_clicked(move |_| {
                let client_clone = Arc::clone(&client_clone);
                glib::MainContext::default().spawn_local(async move {
                    team::create_team(&client_clone, "New College", "New Mascot", 0, 0, 0).await.unwrap();
                });
            });

            window.add(&button);
            window.show_all();
        });
    });

    app.run();
    Ok(())
}