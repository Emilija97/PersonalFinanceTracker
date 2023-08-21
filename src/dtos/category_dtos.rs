use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CategoryInDTO {
    pub name: String,
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryOutDTO {
    pub id: Uuid,
    pub name: String,
    pub user_id: Uuid,
}
