use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{dtos::account_dtos::AccountOutDTO, enums::custom_enums::AccountType};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub account_type: AccountType,
    pub balance: f64,
    pub user_id: Uuid,
}

impl Account {
    pub fn to_account_out_dto(&self) -> AccountOutDTO {
        AccountOutDTO {
            id: self.id,
            name: self.name.clone(),
            account_type: self.account_type.clone(),
            balance: self.balance,
            user_id: self.user_id,
        }
    }
}
