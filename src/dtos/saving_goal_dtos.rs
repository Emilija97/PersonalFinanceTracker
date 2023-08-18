use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SavingGoalInDTO {
    pub title: String,
    pub target_amount: f64,
    pub current_amount: f64,
    pub deadline: chrono::NaiveDate,
    pub user_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct SavingGoalOutDTO {
    pub id: Uuid,
    pub title: String,
    pub target_amount: f64,
    pub current_amount: f64,
    pub deadline: chrono::NaiveDate,
    pub user_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
