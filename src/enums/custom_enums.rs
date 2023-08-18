use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[sqlx(type_name = "transaction_type", rename_all = "snake_case")]
pub enum TransactionType {
    Income,
    Expense,
}

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[sqlx(type_name = "account_type", rename_all = "snake_case")]
pub enum AccountType {
    Bank,
    Cash,
    Card,
}

#[derive(sqlx::Type, Debug, Clone, PartialEq)]
#[sqlx(type_name = "INT4", rename_all = "snake_case")]
pub enum CategoryEnum {
    Groceries = 1,
    Rent = 2,
    Entertainment = 3,
    Pharmacy = 4,
    Utils = 5,
}

impl From<i32> for CategoryEnum {
    fn from(value: i32) -> Self {
        match value {
            1 => CategoryEnum::Groceries,
            2 => CategoryEnum::Rent,
            3 => CategoryEnum::Entertainment,
            4 => CategoryEnum::Pharmacy,
            5 => CategoryEnum::Utils,
            _ => panic!("Invalid value for CategoryEnum"),
        }
    }
}

impl Into<i32> for CategoryEnum {
    fn into(self) -> i32 {
        self as i32
    }
}
