#[derive(Debug)]
pub struct Team {
    pub id: i32,
    pub college: String,
    pub mascot: String,
    pub wins: i32,
    pub losses: i32,
    pub ties: i32,
}

#[derive(Debug)]
pub struct PhysicalScore {
    pub id: i32,
    pub team_id: i32,
    pub score: i32,
    pub avg_height: f32,
    pub avg_weight: f32,
    pub strength_mod: i32,
    pub condition_mod: i32,
}

#[derive(Debug)]
pub struct MentalScore {
    pub id: i32,
    pub team_id: i32,
    pub score: i32,
    pub avg_age: i32,
    pub intelligence_mod: i32,
    pub morale_mod: i32,
    pub focus_mod: i32,
}