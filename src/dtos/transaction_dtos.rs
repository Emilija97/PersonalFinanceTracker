use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::enums::custom_enums::TransactionType;

#[derive(Debug, Deserialize)]
pub struct TransactionInDTO {
    pub title: String,
    pub amount: f64,
    pub date: chrono::NaiveDateTime,
    pub category_id: Uuid,
    pub transaction_type: TransactionType,
    pub user_id: Uuid,
    pub account_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct TransactionOutDTO {
    pub id: Uuid,
    pub title: String,
    pub amount: f64,
    pub date: chrono::NaiveDateTime,
    pub category_id: Uuid,
    pub(crate) transaction_type: TransactionType,
    pub user_id: Uuid,
    pub account_id: Uuid,
}
