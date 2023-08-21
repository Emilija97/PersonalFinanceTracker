use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{dtos::transaction_dtos::TransactionOutDTO, enums::custom_enums::TransactionType};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub title: String,
    pub amount: f64,
    pub date: chrono::NaiveDateTime,
    pub category_id: Uuid,
    pub transaction_type: TransactionType,
    pub user_id: Uuid,
    pub account_id: Uuid,
}

impl Transaction {
    pub fn to_transaction_out_dto(&self) -> TransactionOutDTO {
        TransactionOutDTO {
            id: self.id,
            title: self.title.clone(),
            amount: self.amount,
            date: self.date,
            category_id: self.category_id,
            transaction_type: self.transaction_type,
            user_id: self.user_id,
            account_id: self.account_id,
        }
    }
}
