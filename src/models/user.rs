use crate::dtos::user_dtos::UserOutDTO;

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl User {
    pub fn to_user_out_dto(&self) -> UserOutDTO {
        UserOutDTO {
            id: self.id,
            username: self.username.clone(),
            email: self.email.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
