use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct Account {
    id: Uuid,
    name: String,
    account_type: String,
    balance: f64,
    user_id: Uuid,
}
