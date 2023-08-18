use sqlx::postgres::PgPool;
use std::env;

// pub fn create_pool() -> PgPool {
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     PgPool::connect(&database_url)
//         .await
//         .expect("Failed to create pool")
// }

pub async fn establish_connection(is_test: bool) -> PgPool {
    // Load .env variables
    dotenv::dotenv().ok();

    // Fetch the DATABASE_URL from environment variable
    let database_url_key = if is_test {
        "TEST_DATABASE_URL"
    } else {
        "DATABASE_URL"
    };
    let database_url =
        env::var(database_url_key).expect(&format!("{} must be set", database_url_key));

    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a new connection pool
    PgPool::connect(&database_url)
        .await
        .expect("Failed to create DB pool.")
}
