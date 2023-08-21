use crate::dtos::saving_goal_dtos::{SavingGoalInDTO, SavingGoalOutDTO};
use crate::operations::saving_goal_ops::*;
use crate::uuid_param::UuidParam;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post, routes, Route};
use sqlx::PgPool;

#[get("/saving_goals")]
pub async fn get_all_saving_goals(
    db: &rocket::State<PgPool>,
) -> Result<Json<Vec<SavingGoalOutDTO>>, status::Custom<String>> {
    match fetch_all_saving_goals(db).await {
        Ok(saving_goals) => {
            let saving_goals_dto: Vec<SavingGoalOutDTO> = saving_goals
                .into_iter()
                .map(|saving_goal| saving_goal.to_saving_goal_out_dto())
                .collect();
            Ok(Json(saving_goals_dto))
        }
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to fetch saving_goals.".to_string(),
        )),
    }
}

#[get("/saving_goals/<saving_goal_id_param>")]
pub async fn get_saving_goal_by_id(
    db: &rocket::State<PgPool>,
    saving_goal_id_param: UuidParam,
) -> Result<Json<SavingGoalOutDTO>, status::Custom<String>> {
    let saving_goal_id = saving_goal_id_param.0;
    match find_saving_goal_by_id(db, saving_goal_id).await {
        Ok(Some(saving_goal)) => Ok(Json(saving_goal.to_saving_goal_out_dto())),
        Ok(None) => Err(status::Custom(
            Status::NotFound,
            "SavingGoal not found.".to_string(),
        )),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to fetch saving_goal.".to_string(),
        )),
    }
}

#[post("/saving_goals", data = "<saving_goal_in>")]
pub async fn post_saving_goal(
    db: &rocket::State<PgPool>,
    saving_goal_in: Json<SavingGoalInDTO>,
) -> Result<Json<SavingGoalOutDTO>, status::Custom<String>> {
    match create_saving_goal(db.inner(), &saving_goal_in.0).await {
        Ok(saving_goal) => Ok(Json(saving_goal.to_saving_goal_out_dto())),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to create saving_goal.".to_string(),
        )),
    }
}

#[patch("/saving_goals/<saving_goal_id_param>", data = "<saving_goal_update>")]
pub async fn update_saving_goal(
    db: &rocket::State<PgPool>,
    saving_goal_id_param: UuidParam,
    saving_goal_update: Json<SavingGoalInDTO>,
) -> Result<Json<SavingGoalOutDTO>, status::Custom<String>> {
    let saving_goal_id = saving_goal_id_param.0;
    match update_saving_goal_in_db(db, saving_goal_id, &saving_goal_update.0).await {
        Ok(saving_goal) => Ok(Json(saving_goal.to_saving_goal_out_dto())),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to update saving_goal.".to_string(),
        )),
    }
}

#[delete("/saving_goals/<saving_goal_id_param>")]
pub async fn delete_saving_goal_by_id(
    db: &rocket::State<PgPool>,
    saving_goal_id_param: UuidParam,
) -> Result<status::NoContent, status::Custom<String>> {
    let saving_goal_id = saving_goal_id_param.0;
    match delete_saving_goal(db, saving_goal_id).await {
        Ok(_) => Ok(status::NoContent),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to delete saving_goal.".to_string(),
        )),
    }
}

pub fn saving_goal_routes() -> Vec<Route> {
    routes![
        post_saving_goal,
        get_all_saving_goals,
        get_saving_goal_by_id,
        update_saving_goal,
        delete_saving_goal_by_id
    ]
}
