use crate::dtos::achievement_dtos::{AchievementInDTO, AchievementOutDTO};
use crate::operations::achievement_ops::*;
use crate::uuid_param::UuidParam;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post, routes, Route};
use sqlx::PgPool;

#[get("/achievements")]
pub async fn get_all_achievements(
    db: &rocket::State<PgPool>,
) -> Result<Json<Vec<AchievementOutDTO>>, status::Custom<String>> {
    match fetch_all_achievements(db).await {
        Ok(achievements) => {
            let achievements_dto: Vec<AchievementOutDTO> = achievements
                .into_iter()
                .map(|achievement| achievement.to_achievement_out_dto())
                .collect();
            Ok(Json(achievements_dto))
        }
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to fetch achievements.".to_string(),
        )),
    }
}

#[get("/achievements/<achievement_id_param>")]
pub async fn get_achievement_by_id(
    db: &rocket::State<PgPool>,
    achievement_id_param: UuidParam,
) -> Result<Json<AchievementOutDTO>, status::Custom<String>> {
    let achievement_id = achievement_id_param.0;
    match find_achievement_by_id(db, achievement_id).await {
        Ok(Some(achievement)) => Ok(Json(achievement.to_achievement_out_dto())),
        Ok(None) => Err(status::Custom(
            Status::NotFound,
            "Achievement not found.".to_string(),
        )),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to fetch achievement.".to_string(),
        )),
    }
}

#[post("/achievements", data = "<achievement_in>")]
pub async fn post_achievement(
    db: &rocket::State<PgPool>,
    achievement_in: Json<AchievementInDTO>,
) -> Result<Json<AchievementOutDTO>, status::Custom<String>> {
    match create_achievement(db.inner(), &achievement_in.0).await {
        Ok(achievement) => Ok(Json(achievement.to_achievement_out_dto())),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to create achievement.".to_string(),
        )),
    }
}

#[patch("/achievements/<achievement_id_param>", data = "<achievement_update>")]
pub async fn update_achievement(
    db: &rocket::State<PgPool>,
    achievement_id_param: UuidParam,
    achievement_update: Json<AchievementInDTO>,
) -> Result<Json<AchievementOutDTO>, status::Custom<String>> {
    let achievement_id = achievement_id_param.0;
    match update_achievement_in_db(db, achievement_id, &achievement_update.0).await {
        Ok(achievement) => Ok(Json(achievement.to_achievement_out_dto())),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to update achievement.".to_string(),
        )),
    }
}

#[delete("/achievements/<achievement_id_param>")]
pub async fn delete_achievement_by_id(
    db: &rocket::State<PgPool>,
    achievement_id_param: UuidParam,
) -> Result<status::NoContent, status::Custom<String>> {
    let achievement_id = achievement_id_param.0;
    match delete_achievement(db, achievement_id).await {
        Ok(_) => Ok(status::NoContent),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to delete achievement.".to_string(),
        )),
    }
}

pub fn achievement_routes() -> Vec<Route> {
    routes![
        post_achievement,
        get_all_achievements,
        get_achievement_by_id,
        update_achievement,
        delete_achievement_by_id
    ]
}
