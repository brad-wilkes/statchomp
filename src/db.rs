// src/db.rs
use tokio_postgres::{NoTls, Error};
use tokio_postgres::tls::NoTlsStream;

pub async fn connect_to_db() -> Result<(tokio_postgres::Client, tokio_postgres::Connection<tokio_postgres::Socket, NoTlsStream>), Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=password dbname=practicedb", NoTls).await?;
    Ok((client, connection))
}