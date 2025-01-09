use tokio_postgres::{Client, Error};
use crate::models::MentalScore;

pub async fn create_mental_score(client: &Client, team_id: i32, score: i32, avg_age: i32, intelligence_mod: i32, morale_mod: i32, focus_mod: i32) -> Result<(), Error> {
    client.execute(
        "INSERT INTO mental_scores (team_id, score, avg_age, intelligence_mod, morale_mod, focus_mod) VALUES ($1, $2, $3, $4, $5, $6)",
        &[&team_id, &score, &avg_age, &intelligence_mod, &morale_mod, &focus_mod],
    ).await?;
    Ok(())
}

pub async fn get_mental_scores(client: &Client) -> Result<Vec<MentalScore>, Error> {
    let rows = client.query("SELECT id, team_id, score, avg_age, intelligence_mod, morale_mod, focus_mod FROM mental_scores", &[]).await?;
    let mut scores = Vec::new();
    for row in rows {
        scores.push(MentalScore {
            id: row.get(0),
            team_id: row.get(1),
            score: row.get(2),
            avg_age: row.get(3),
            intelligence_mod: row.get(4),
            morale_mod: row.get(5),
            focus_mod: row.get(6),
        });
    }
    Ok(scores)
}

pub async fn update_mental_score(client: &Client, id: i32, team_id: i32, score: i32, avg_age: i32, intelligence_mod: i32, morale_mod: i32, focus_mod: i32) -> Result<(), Error> {
    client.execute(
        "UPDATE mental_scores SET team_id = $1, score = $2, avg_age = $3, intelligence_mod = $4, morale_mod = $5, focus_mod = $6 WHERE id = $7",
        &[&team_id, &score, &avg_age, &intelligence_mod, &morale_mod, &focus_mod, &id],
    ).await?;
    Ok(())
}