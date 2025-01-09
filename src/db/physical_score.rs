use tokio_postgres::{Client, Error};
use crate::models::PhysicalScore;

pub async fn create_physical_score(client: &Client, team_id: i32, score: i32, avg_height: f32, avg_weight: f32, strength_mod: i32, condition_mod: i32) -> Result<(), Error> {
    client.execute(
        "INSERT INTO physical_scores (team_id, score, avg_height, avg_weight, strength_mod, condition_mod) VALUES ($1, $2, $3, $4, $5, $6)",
        &[&team_id, &score, &avg_height, &avg_weight, &strength_mod, &condition_mod],
    ).await?;
    Ok(())
}

pub async fn get_physical_scores(client: &Client) -> Result<Vec<PhysicalScore>, Error> {
    let rows = client.query("SELECT id, team_id, score, avg_height, avg_weight, strength_mod, condition_mod FROM physical_scores", &[]).await?;
    let mut scores = Vec::new();
    for row in rows {
        scores.push(PhysicalScore {
            id: row.get(0),
            team_id: row.get(1),
            score: row.get(2),
            avg_height: row.get(3),
            avg_weight: row.get(4),
            strength_mod: row.get(5),
            condition_mod: row.get(6),
        });
    }
    Ok(scores)
}

pub async fn update_physical_score(client: &Client, id: i32, team_id: i32, score: i32, avg_height: f32, avg_weight: f32, strength_mod: i32, condition_mod: i32) -> Result<(), Error> {
    client.execute(
        "UPDATE physical_scores SET team_id = $1, score = $2, avg_height = $3, avg_weight = $4, strength_mod = $5, condition_mod = $6 WHERE id = $7",
        &[&team_id, &score, &avg_height, &avg_weight, &strength_mod, &condition_mod, &id],
    ).await?;
    Ok(())
}