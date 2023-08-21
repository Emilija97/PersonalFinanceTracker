pub mod db;
pub mod dtos;
pub mod enums;
pub mod models;
pub mod operations;
pub mod routes;
pub mod uuid_param;
// // ... other modules ...
// #[macro_use]
// extern crate rocket;

use rocket::Build;
use routes::{
    account_routes::account_routes, budget_routes::budget_routes, category_routes::category_routes,
    transaction_routes::transaction_routes, user_routes::*,
};
use sqlx::PgPool;

pub fn create_rocket(pool: PgPool) -> rocket::Rocket<Build> {
    rocket::build()
        .manage(pool)
        .mount("/", user_routes())
        .mount("/accounts", account_routes())
        .mount("/", transaction_routes())
        .mount("/", category_routes())
        .mount("/", budget_routes())
}
