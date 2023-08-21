use crate::dtos::user_dtos::{UserInDTO, UserOutDTO};
use crate::operations::user_ops::*;
// use personal_finance_tracker::dtos::user_dtos::{UserInDTO, UserOutDTO};
use crate::uuid_param::UuidParam;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post, routes, Route};
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

#[get("/users/<user_id_param>")]
pub async fn get_user_by_id(
    db: &rocket::State<PgPool>,
    user_id_param: UuidParam,
) -> Result<Json<UserOutDTO>, status::Custom<String>> {
    let user_id = user_id_param.0;
    match find_user_by_id(db, user_id).await {
        Ok(Some(user)) => Ok(Json(user.to_user_out_dto())),
        Ok(None) => Err(status::Custom(
            Status::NotFound,
            "User not found.".to_string(),
        )),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to fetch user.".to_string(),
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

#[patch("/users/<user_id_param>", data = "<user_update>")]
pub async fn update_user(
    db: &rocket::State<PgPool>,
    user_id_param: UuidParam,
    user_update: Json<UserInDTO>,
) -> Result<Json<UserOutDTO>, status::Custom<String>> {
    let user_id = user_id_param.0;
    match update_user_in_db(db, user_id, &user_update.0).await {
        Ok(user) => Ok(Json(user.to_user_out_dto())),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to update user.".to_string(),
        )),
    }
}

#[delete("/users/<user_id_param>")]
pub async fn delete_user_by_id(
    db: &rocket::State<PgPool>,
    user_id_param: UuidParam,
) -> Result<status::NoContent, status::Custom<String>> {
    let user_id = user_id_param.0;
    match delete_user(db, user_id).await {
        Ok(_) => Ok(status::NoContent),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to delete user.".to_string(),
        )),
    }
}

pub fn user_routes() -> Vec<Route> {
    routes![
        post_user,
        get_all_users,
        get_user_by_id,
        update_user,
        delete_user_by_id
    ]
}
