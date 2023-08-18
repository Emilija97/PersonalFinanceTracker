use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    id: uuid::Uuid,
    name: String,
    user_id: uuid::Uuid,
}
