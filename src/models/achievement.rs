use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct Achievement {
    id: Uuid,
    goal_id: Uuid,
    date_achieved: chrono::NaiveDateTime,
    amount_saved: f64,
}
