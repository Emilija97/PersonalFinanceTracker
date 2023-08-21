use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dtos::saving_goal_dtos::SavingGoalOutDTO;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SavingGoal {
    pub id: Uuid,
    pub title: String,
    pub target_amount: f64,
    pub current_amount: f64,
    pub deadline: chrono::NaiveDate,
    pub user_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl SavingGoal {
    pub fn to_saving_goal_out_dto(&self) -> SavingGoalOutDTO {
        SavingGoalOutDTO {
            id: self.id,
            title: self.title.clone(),
            target_amount: self.target_amount,
            current_amount: self.current_amount,
            deadline: self.deadline,
            user_id: self.user_id,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
