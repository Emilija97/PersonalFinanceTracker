use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct SavingGoal {
    id: Uuid,
    title: String,
    target_amount: f64,
    current_amount: f64,
    deadline: chrono::NaiveDate,
    user_id: Uuid,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}
