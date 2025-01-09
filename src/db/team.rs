use tokio_postgres::{Client, Error};
use crate::models::Team;

pub async fn create_team(client: &Client, college: &str, mascot: &str, wins: i32, losses: i32, ties: i32) -> Result<(), Error> {
    client.execute(
        "INSERT INTO teams (college, mascot, wins, losses, ties) VALUES ($1, $2, $3, $4, $5)",
        &[&college, &mascot, &wins, &losses, &ties],
    ).await?;
    Ok(())
}

pub async fn get_teams(client: &Client) -> Result<Vec<Team>, Error> {
    let rows = client.query("SELECT id, college, mascot, wins, losses, ties FROM teams", &[]).await?;
    let mut teams = Vec::new();
    for row in rows {
        teams.push(Team {
            id: row.get(0),
            college: row.get(1),
            mascot: row.get(2),
            wins: row.get(3),
            losses: row.get(4),
            ties: row.get(5),
        });
    }
    Ok(teams)
}

pub async fn update_team(client: &Client, id: i32, college: &str, mascot: &str, wins: i32, losses: i32, ties: i32) -> Result<(), Error> {
    client.execute(
        "UPDATE teams SET college = $1, mascot = $2, wins = $3, losses = $4, ties = $5 WHERE id = $6",
        &[&college, &mascot, &wins, &losses, &ties, &id],
    ).await?;
    Ok(())
}