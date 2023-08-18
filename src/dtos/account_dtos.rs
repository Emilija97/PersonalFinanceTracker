use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct AccountInDTO {
    pub name: String,
    pub balance: f64,
    pub account_type: String,
    pub user_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct AccountOutDTO {
    pub id: Uuid,
    pub name: String,
    pub balance: f64,
    pub account_type: String,
    pub user_id: Uuid,
}
