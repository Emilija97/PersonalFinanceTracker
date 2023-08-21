use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dtos::budget_dtos::BudgetOutDTO;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Budget {
    pub id: Uuid,
    pub name: String,
    pub amount: f64,
    pub start_date: chrono::NaiveDateTime,
    pub end_date: chrono::NaiveDateTime,
    pub user_id: Uuid,
    pub category_id: Option<Uuid>,
}

impl Budget {
    pub fn to_budget_out_dto(&self) -> BudgetOutDTO {
        BudgetOutDTO {
            id: self.id,
            name: self.name.clone(),
            amount: self.amount,
            start_date: self.start_date,
            end_date: self.end_date,
            user_id: self.user_id,
            category_id: self.category_id,
        }
    }
}
