#[macro_use]
extern crate rocket;

// mod db;
// mod dtos;
// mod enums;
// mod models;
// mod operations;
// mod routes;

use db::conn::establish_connection;
use personal_finance_tracker::create_rocket;
use personal_finance_tracker::db;
// use personal_finance_tracker::routes;
// use routes::user_routes;
// use personal_finance_tracker::create_rocket;
// use personal_finance_tracker::;
// use rocket::Build;
// use routes::user_routes;
// use sqlx::PgPool;

// #[get("/")]
// pub fn hello() -> &'static str {
//     "Hello, world!"
// }

// #[launch]
// async fn rocket() -> _ {
//     let pool = establish_connection(false).await;
//     rocket::build()
//         .manage(pool)
//         .mount("/", routes![hello])
//         .mount("/", user_routes::user_routes())
// }

// pub async fn create_rocket(pool: PgPool) -> rocket::Rocket<Build> {
//     rocket::build()
//         .manage(pool)
//         .mount("/", routes![hello])
//         .mount("/", user_routes::user_routes())
// }

#[launch]
async fn rocket() -> _ {
    let pool = establish_connection(false).await; // false means it's not for tests
    create_rocket(pool)
}
