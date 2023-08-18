use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{dtos::transaction_dtos::TransactionOutDTO, enums::custom_enums::TransactionType};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Transaction {
    id: Uuid,
    title: String,
    amount: f64,
    date: chrono::NaiveDateTime,
    category_id: Uuid,
    transaction_type: TransactionType,
    user_id: Uuid,
    account_id: Uuid,
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
