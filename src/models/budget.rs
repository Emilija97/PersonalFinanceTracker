use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
struct Budget {
    id: Uuid,
    name: String,
    amount: f64,
    start_date: chrono::NaiveDateTime,
    end_date: chrono::NaiveDateTime,
    user_id: Uuid,
    category_id: Option<Uuid>,
}
