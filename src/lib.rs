pub mod db;
pub mod dtos;
pub mod enums;
pub mod models;
pub mod operations;
pub mod routes;
// // ... other modules ...
// #[macro_use]
// extern crate rocket;

use rocket::Build;
use routes::user_routes::*;
use sqlx::PgPool;

pub fn create_rocket(pool: PgPool) -> rocket::Rocket<Build> {
    rocket::build().manage(pool).mount("/", user_routes())
    // ... other rocket configurations ...
}
