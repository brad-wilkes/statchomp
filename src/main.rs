mod db;
mod gui;

use tokio_postgres::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (_client, connection) = db::connect_to_db().await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    gui::create_and_run_app().expect("TODO: panic message");

    Ok(())
}