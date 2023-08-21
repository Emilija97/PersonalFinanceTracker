use crate::dtos::budget_dtos::{BudgetInDTO, BudgetOutDTO};
use crate::operations::budget_ops::*;
use crate::uuid_param::UuidParam;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post, routes, Route};
use sqlx::PgPool;

#[get("/budgets")]
pub async fn get_all_budgets(
    db: &rocket::State<PgPool>,
) -> Result<Json<Vec<BudgetOutDTO>>, status::Custom<String>> {
    match fetch_all_budgets(db).await {
        Ok(budgets) => {
            let budgets_dto: Vec<BudgetOutDTO> = budgets
                .into_iter()
                .map(|budget| budget.to_budget_out_dto())
                .collect();
            Ok(Json(budgets_dto))
        }
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to fetch budgets.".to_string(),
        )),
    }
}

#[get("/budgets/<budget_id_param>")]
pub async fn get_budget_by_id(
    db: &rocket::State<PgPool>,
    budget_id_param: UuidParam,
) -> Result<Json<BudgetOutDTO>, status::Custom<String>> {
    let budget_id = budget_id_param.0;
    match find_budget_by_id(db, budget_id).await {
        Ok(Some(budget)) => Ok(Json(budget.to_budget_out_dto())),
        Ok(None) => Err(status::Custom(
            Status::NotFound,
            "Budget not found.".to_string(),
        )),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to fetch budget.".to_string(),
        )),
    }
}

#[post("/budgets", data = "<budget_in>")]
pub async fn post_budget(
    db: &rocket::State<PgPool>,
    budget_in: Json<BudgetInDTO>,
) -> Result<Json<BudgetOutDTO>, status::Custom<String>> {
    match create_budget(db.inner(), &budget_in.0).await {
        Ok(budget) => Ok(Json(budget.to_budget_out_dto())),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to create budget.".to_string(),
        )),
    }
}

#[patch("/budgets/<budget_id_param>", data = "<budget_update>")]
pub async fn update_budget(
    db: &rocket::State<PgPool>,
    budget_id_param: UuidParam,
    budget_update: Json<BudgetInDTO>,
) -> Result<Json<BudgetOutDTO>, status::Custom<String>> {
    let budget_id = budget_id_param.0;
    match update_budget_in_db(db, budget_id, &budget_update.0).await {
        Ok(budget) => Ok(Json(budget.to_budget_out_dto())),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to update budget.".to_string(),
        )),
    }
}

#[delete("/budgets/<budget_id_param>")]
pub async fn delete_budget_by_id(
    db: &rocket::State<PgPool>,
    budget_id_param: UuidParam,
) -> Result<status::NoContent, status::Custom<String>> {
    let budget_id = budget_id_param.0;
    match delete_budget(db, budget_id).await {
        Ok(_) => Ok(status::NoContent),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to delete budget.".to_string(),
        )),
    }
}

pub fn budget_routes() -> Vec<Route> {
    routes![
        post_budget,
        get_all_budgets,
        get_budget_by_id,
        update_budget,
        delete_budget_by_id
    ]
}
