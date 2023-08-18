#[derive(Debug, rocket::serde::Deserialize)]
pub struct UserInDTO {
    pub username: String,
    pub email: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserOutDTO {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
