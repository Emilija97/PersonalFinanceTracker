use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct BudgetInDTO {
    pub name: String,
    pub amount: f64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Debug, Serialize)]
pub struct BudgetOutDTO {
    pub id: Uuid,
    pub name: String,
    pub amount: f64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}
