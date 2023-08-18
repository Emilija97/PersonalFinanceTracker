use crate::dtos::user_dtos::{UserInDTO, UserOutDTO};
use crate::operations::user_ops::*;
// use personal_finance_tracker::dtos::user_dtos::{UserInDTO, UserOutDTO};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{get, post, routes, Route};
use sqlx::PgPool;

#[get("/users")]
pub async fn get_all_users(
    db: &rocket::State<PgPool>,
) -> Result<Json<Vec<UserOutDTO>>, status::Custom<String>> {
    match fetch_all_users(db).await {
        Ok(users) => {
            let users_dto: Vec<UserOutDTO> = users
                .into_iter()
                .map(|user| user.to_user_out_dto())
                .collect();
            Ok(Json(users_dto))
        }
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to fetch users.".to_string(),
        )),
    }
}

#[post("/users", data = "<user_in>")]
pub async fn post_user(
    db: &rocket::State<PgPool>,
    user_in: Json<UserInDTO>,
) -> Result<Json<UserOutDTO>, status::Custom<String>> {
    match create_user(db.inner(), &user_in.0).await {
        Ok(user) => Ok(Json(user.to_user_out_dto())),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to create user.".to_string(),
        )),
    }
}

pub fn user_routes() -> Vec<Route> {
    routes![post_user, get_all_users]
}
