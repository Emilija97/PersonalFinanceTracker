use crate::dtos::category_dtos::{CategoryInDTO, CategoryOutDTO};
use crate::operations::category_ops::*;
// use personal_finance_tracker::dtos::category_dtos::{CategoryInDTO, CategoryOutDTO};
use crate::uuid_param::UuidParam;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post, routes, Route};
use sqlx::PgPool;

#[get("/categories")]
pub async fn get_all_categories(
    db: &rocket::State<PgPool>,
) -> Result<Json<Vec<CategoryOutDTO>>, status::Custom<String>> {
    match fetch_all_categories(db).await {
        Ok(categories) => {
            let categories_dto: Vec<CategoryOutDTO> = categories
                .into_iter()
                .map(|category| category.to_category_out_dto())
                .collect();
            Ok(Json(categories_dto))
        }
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to fetch categories.".to_string(),
        )),
    }
}

#[get("/categories/<category_id_param>")]
pub async fn get_category_by_id(
    db: &rocket::State<PgPool>,
    category_id_param: UuidParam,
) -> Result<Json<CategoryOutDTO>, status::Custom<String>> {
    let category_id = category_id_param.0;
    match find_category_by_id(db, category_id).await {
        Ok(Some(category)) => Ok(Json(category.to_category_out_dto())),
        Ok(None) => Err(status::Custom(
            Status::NotFound,
            "Category not found.".to_string(),
        )),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to fetch category.".to_string(),
        )),
    }
}

#[post("/categories", data = "<category_in>")]
pub async fn post_category(
    db: &rocket::State<PgPool>,
    category_in: Json<CategoryInDTO>,
) -> Result<Json<CategoryOutDTO>, status::Custom<String>> {
    match create_category(db.inner(), &category_in.0).await {
        Ok(category) => Ok(Json(category.to_category_out_dto())),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to create category.".to_string(),
        )),
    }
}

#[patch("/categories/<category_id_param>", data = "<category_update>")]
pub async fn update_category(
    db: &rocket::State<PgPool>,
    category_id_param: UuidParam,
    category_update: Json<CategoryInDTO>,
) -> Result<Json<CategoryOutDTO>, status::Custom<String>> {
    let category_id = category_id_param.0;
    match update_category_in_db(db, category_id, &category_update.0).await {
        Ok(category) => Ok(Json(category.to_category_out_dto())),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to update category.".to_string(),
        )),
    }
}

#[delete("/categories/<category_id_param>")]
pub async fn delete_category_by_id(
    db: &rocket::State<PgPool>,
    category_id_param: UuidParam,
) -> Result<status::NoContent, status::Custom<String>> {
    let category_id = category_id_param.0;
    match delete_category(db, category_id).await {
        Ok(_) => Ok(status::NoContent),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to delete category.".to_string(),
        )),
    }
}

pub fn category_routes() -> Vec<Route> {
    routes![
        post_category,
        get_all_categories,
        get_category_by_id,
        update_category,
        delete_category_by_id
    ]
}
