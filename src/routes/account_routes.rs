use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post, routes, Route, State};
use sqlx::PgPool;

use crate::dtos::account_dtos::{AccountInDTO, AccountOutDTO};
use crate::operations::account_ops::*;
use crate::uuid_param::UuidParam;

#[get("/")]
async fn read_all(db: &State<PgPool>) -> Result<Json<Vec<AccountOutDTO>>, status::Custom<String>> {
    match get_all_accounts(db).await {
        Ok(accounts) => {
            let accounts_dto: Vec<AccountOutDTO> = accounts
                .into_iter()
                .map(|account| account.to_account_out_dto())
                .collect();
            Ok(Json(accounts_dto))
        }
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to fetch accounts.".to_string(),
        )),
    }
}

#[get("/<account_id_param>")]
pub async fn get_account_by_id(
    db: &rocket::State<PgPool>,
    account_id_param: UuidParam,
) -> Result<Json<AccountOutDTO>, status::Custom<String>> {
    let account_id = account_id_param.0;
    match find_account_by_id(db, account_id).await {
        Ok(Some(account)) => Ok(Json(account.to_account_out_dto())),
        Ok(None) => Err(status::Custom(
            Status::NotFound,
            "Account not found.".to_string(),
        )),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to fetch account.".to_string(),
        )),
    }
}

#[post("/", data = "<account_in>")]
pub async fn post_account(
    db: &rocket::State<PgPool>,
    account_in: Json<AccountInDTO>,
) -> Result<Json<AccountOutDTO>, status::Custom<String>> {
    match create_account(db.inner(), &account_in.0).await {
        Ok(account) => Ok(Json(account.to_account_out_dto())),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to create account.".to_string(),
        )),
    }
}

#[patch("/<account_id_param>", data = "<account_in>")]
pub async fn patch_account(
    db: &rocket::State<PgPool>,
    account_id_param: UuidParam,
    account_in: Json<AccountInDTO>,
) -> Result<Json<AccountOutDTO>, status::Custom<String>> {
    let account_id = account_id_param.0;
    match update_account(db, account_id, &account_in.0).await {
        Ok(account) => Ok(Json(account.to_account_out_dto())),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to update account.".to_string(),
        )),
    }
}

#[delete("/<account_id_param>")]
pub async fn delete_account_route(
    db: &rocket::State<PgPool>,
    account_id_param: UuidParam,
) -> Result<status::NoContent, status::Custom<String>> {
    let account_id = account_id_param.0;
    match delete_account(db, account_id).await {
        Ok(_) => Ok(status::NoContent),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to delete account.".to_string(),
        )),
    }
}

pub fn account_routes() -> Vec<Route> {
    routes![
        read_all,
        get_account_by_id,
        post_account,
        patch_account,
        delete_account_route
    ]
}
