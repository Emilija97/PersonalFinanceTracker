// // use once_cell::sync::Lazy;
// // use rocket::local::asynchronous::Client;
// // use sqlx::{Pool, Postgres};

// // // Connection string for your test database.
// // const TEST_DB_URL: &str = "postgresql://username:password@localhost/test_db_name";

// // // Global instance of the test database connection pool.
// // pub static TEST_DB_POOL: Lazy<Pool<Postgres>> = Lazy::new(|| {
// //     Pool::<Postgres>::connect_lazy(TEST_DB_URL).expect("Failed to create pool for test DB")
// // });

// // pub async fn get_rocket_client() -> Client {
// //     let rocket_instance =
// //         rocket::build().mount("/api", rocket::routes![crate::routes::your_endpoint]);
// //     Client::tracked(rocket_instance)
// //         .await
// //         .expect("Valid rocket instance")
// // }

// // pub async fn clean_database() {
// //     // Clean up the test database after each test.
// //     let mut conn = TEST_DB_POOL
// //         .acquire()
// //         .await
// //         .expect("Failed to get test DB connection");
// //     sqlx::query!("TRUNCATE your_table_name CASCADE")
// //         .execute(&mut conn)
// //         .await
// //         .unwrap();
// //     // You can add more TRUNCATE commands for other tables as required.
// // }

// // extern crate personal_finance_tracker;
use personal_finance_tracker::{create_rocket, db};
use rocket::local::asynchronous::Client;

pub async fn setup() -> (Client, sqlx::Pool<sqlx::Postgres>) {
    let pool: sqlx::Pool<sqlx::Postgres> = db::conn::establish_connection(true).await;
    let rocket = create_rocket(pool.clone());
    let client = Client::tracked(rocket)
        .await
        .expect("valid rocket instance");
    (client, pool)
}
