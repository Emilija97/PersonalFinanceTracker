use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct AchievementInDTO {
    pub goal_id: Uuid,
    pub date_achieved: chrono::NaiveDateTime,
    pub amount_saved: f64,
}

#[derive(Debug, Serialize)]
pub struct AchievementOutDTO {
    pub id: Uuid,
    pub goal_id: Uuid,
    pub date_achieved: chrono::NaiveDateTime,
    pub amount_saved: f64,
}
