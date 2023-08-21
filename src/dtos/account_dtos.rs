use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::enums::custom_enums::AccountType;

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountInDTO {
    pub name: String,
    pub balance: f64,
    pub account_type: AccountType,
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountOutDTO {
    pub id: Uuid,
    pub name: String,
    pub balance: f64,
    pub account_type: AccountType,
    pub user_id: Uuid,
}
