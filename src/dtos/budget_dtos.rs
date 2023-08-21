use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct BudgetInDTO {
    pub name: String,
    pub amount: f64,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub user_id: Uuid,
    pub category_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetOutDTO {
    pub id: Uuid,
    pub name: String,
    pub amount: f64,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub user_id: Uuid,
    pub category_id: Option<Uuid>,
}
