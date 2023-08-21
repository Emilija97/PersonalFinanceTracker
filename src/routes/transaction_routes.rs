use crate::dtos::transaction_dtos::{TransactionInDTO, TransactionOutDTO};
use crate::operations::transaction_ops::*;
use crate::uuid_param::UuidParam;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post, routes, Route};
use sqlx::PgPool;

#[get("/transactions")]
pub async fn get_all_transactions(
    db: &rocket::State<PgPool>,
) -> Result<Json<Vec<TransactionOutDTO>>, status::Custom<String>> {
    match fetch_all_transactions(db).await {
        Ok(transactions) => {
            let transactions_dto: Vec<TransactionOutDTO> = transactions
                .into_iter()
                .map(|transaction| transaction.to_transaction_out_dto())
                .collect();
            Ok(Json(transactions_dto))
        }
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to fetch transactions.".to_string(),
        )),
    }
}

#[get("/transactions/<transaction_id_param>")]
pub async fn get_transaction_by_id(
    db: &rocket::State<PgPool>,
    transaction_id_param: UuidParam,
) -> Result<Json<TransactionOutDTO>, status::Custom<String>> {
    let transaction_id = transaction_id_param.0;
    match find_transaction_by_id(db, transaction_id).await {
        Ok(Some(transaction)) => Ok(Json(transaction.to_transaction_out_dto())),
        Ok(None) => Err(status::Custom(
            Status::NotFound,
            "Transaction not found.".to_string(),
        )),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to fetch transaction.".to_string(),
        )),
    }
}

#[post("/transactions", data = "<transaction_in>")]
pub async fn post_transaction(
    db: &rocket::State<PgPool>,
    transaction_in: Json<TransactionInDTO>,
) -> Result<Json<TransactionOutDTO>, status::Custom<String>> {
    match create_transaction(db.inner(), &transaction_in.0).await {
        Ok(transaction) => Ok(Json(transaction.to_transaction_out_dto())),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to create transaction.".to_string(),
        )),
    }
}

#[patch("/transactions/<transaction_id_param>", data = "<transaction_in>")]
pub async fn patch_transaction(
    db: &rocket::State<PgPool>,
    transaction_id_param: UuidParam,
    transaction_in: Json<TransactionInDTO>,
) -> Result<Json<TransactionOutDTO>, status::Custom<String>> {
    let transaction_id = transaction_id_param.0;
    match update_transaction(db, transaction_id, &transaction_in.0).await {
        Ok(transaction) => Ok(Json(transaction.to_transaction_out_dto())),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to update transaction.".to_string(),
        )),
    }
}

#[delete("/transactions/<transaction_id_param>")]
pub async fn delete_transaction_route(
    db: &rocket::State<PgPool>,
    transaction_id_param: UuidParam,
) -> Result<status::NoContent, status::Custom<String>> {
    let transaction_id = transaction_id_param.0;
    match delete_transaction(db, transaction_id).await {
        Ok(_) => Ok(status::NoContent),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to delete".to_string(),
        )),
    }
}

pub fn transaction_routes() -> Vec<Route> {
    routes![
        get_all_transactions,
        get_transaction_by_id,
        post_transaction,
        patch_transaction,
        delete_transaction_route
    ]
}
