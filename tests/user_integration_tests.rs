use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::Client;
use rocket::serde::json::serde_json;
use serde_json::json;

use personal_finance_tracker::create_rocket;
use personal_finance_tracker::db::conn::establish_connection;
use personal_finance_tracker::dtos::user_dtos::UserOutDTO;

#[rocket::async_test]
async fn create_user_integration_test() {
    // 1. Setup a Rocket instance with a test database.
    let pool = establish_connection(true).await; // Assuming 'true' indicates it's for tests
    let rocket = create_rocket(pool);
    let client = Client::untracked(rocket)
        .await
        .expect("valid rocket instance");

    // 2. Create a sample user DTO.
    let user_data = json!({
        "username": "test_username",
        "email": "test_email@example.com"
    });

    // 3. Make a request to the post_user route.
    let response = client
        .post("/users")
        .header(ContentType::JSON)
        .body(user_data.to_string())
        .dispatch()
        .await;

    // 4. Check the response to ensure the user was correctly added.
    assert_eq!(response.status(), Status::Ok);

    let response_body = response.into_string().await.expect("Response has a body");
    let user_out: UserOutDTO = serde_json::from_str(&response_body).expect("Valid UserOutDTO");

    assert_eq!(user_out.username, "test_username");
    assert_eq!(user_out.email, "test_email@example.com");
}

// // use super::*;
// // use rocket::local::blocking::Client;

// use crate::common::setup;
// use personal_finance_tracker::{dtos::user_dtos::UserInDTO, operations::user_ops::create_user};

// #[test]
// fn test_create_user() {
//     // Setup the Rocket instance and database connection
//     // Setup the Rocket instance
//     // let rocket = rocket_test_instance();
//     // let client = Client::tracked(rocket).expect("valid rocket instance");

//     let client = setup();

//     // Create a test user
//     let user_dto = UserInDTO {
//         username: "testuser".to_string(),
//         email: "testuser@example.com".to_string(),
//     };

//     let response = create_user(&pool, &user_dto).await;

//     assert!(response.is_ok());

//     let user = response.unwrap();
//     assert_eq!(user.username, "testuser");
//     assert_eq!(user.email, "testuser@example.com");

//     // Clean up after test (e.g., remove the test user)
// }

// mod common;

// #[cfg(test)]
// mod tests {

//     use rocket::http::{ContentType, Status};
//     use rocket::serde::json::json;

//     use crate::common::setup;

//     #[rocket::async_test]
//     async fn create_user_test() {
//         let client = setup().await;

//         let user = json!({
//             "username": "testuser",
//             "email": "testuser@example.com",
//         });

//         let response = client
//             .post("/users")
//             .header(ContentType::JSON)
//             .body(user.to_string())
//             .dispatch();

//         println!("{}", response.status());
//         assert_eq!(response.status(), Status::Ok);

//         let response_body = response.into_string();

//         // Here you can make more specific assertions about the response body if necessary.
//         // For example, you can parse it as JSON and check specific fields.
//         // println!("Response: {}", response_body);
//     }
// }

// tests/user_tests.rs

// use personal_finance_tracker::dtos::user_dtos::UserInDTO;
// use personal_finance_tracker::operations::user_ops::create_user;
// use rocket::tokio;
// use sqlx::{Pool, Postgres};

// #[tokio::test]
// async fn test_create_user() {
//     // Step 1: Setup
//     let database_url = "your_test_database_url_here"; // Make sure this points to your TEST database.
//     let pool = Pool::<Postgres>::connect(&database_url).await.unwrap();

//     // Run migrations if you have any (this ensures your test database schema is up-to-date)
//     migrate::run(&pool).await.unwrap();

//     let test_user = UserInDTO {
//         username: "testuser".to_string(),
//         email: "testuser@example.com".to_string(),
//     };

//     // Step 2: Act
//     let result = create_user(&pool, &test_user).await;

//     // Step 3: Assert
//     assert!(result.is_ok());
//     let user = result.unwrap();
//     assert_eq!(user.username, test_user.username);
//     assert_eq!(user.email, test_user.email);

//     // Step 4: Clean-up (optional, but useful to avoid leftover data)
//     sqlx::query("DELETE FROM users WHERE username = 'testuser'")
//         .execute(&pool)
//         .await
//         .unwrap();
// }
