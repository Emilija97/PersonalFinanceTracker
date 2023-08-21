use serde::{Deserialize, Serialize};

use crate::dtos::category_dtos::CategoryOutDTO;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    pub id: uuid::Uuid,
    pub name: String,
    pub user_id: uuid::Uuid,
}

impl Category {
    pub fn to_category_out_dto(&self) -> CategoryOutDTO {
        CategoryOutDTO {
            id: self.id,
            name: self.name.clone(),
            user_id: self.user_id,
        }
    }
}
