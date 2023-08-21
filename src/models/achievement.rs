use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dtos::achievement_dtos::AchievementOutDTO;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Achievement {
    pub id: Uuid,
    pub goal_id: Uuid,
    pub date_achieved: chrono::NaiveDateTime,
    pub amount_saved: f64,
}

impl Achievement {
    pub fn to_achievement_out_dto(&self) -> AchievementOutDTO {
        AchievementOutDTO {
            id: self.id,
            goal_id: self.goal_id,
            date_achieved: self.date_achieved,
            amount_saved: self.amount_saved,
        }
    }
}
