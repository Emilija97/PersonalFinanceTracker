use personal_finance_tracker::operations::user_ops::create_user;
use rocket::http::{ContentType, Status};
use rocket::serde::json::serde_json;
use serde_json::json;

use personal_finance_tracker::dtos::user_dtos::{UserInDTO, UserOutDTO};
use sqlx::PgPool;

use crate::common::setup;

mod common;

async fn cleanup_test_user(pool: &PgPool, username: &str, email: &str) {
    sqlx::query("DELETE FROM users WHERE username = $1 AND email = $2")
        .bind(username)
        .bind(email)
        .execute(pool)
        .await
        .expect("Failed to cleanup test user");
}

#[rocket::async_test]
async fn create_user_integration_test() {
    let (client, pool) = setup().await;

    let user_data = json!({
        "username": "test_username",
        "email": "test_email@example.com"
    });

    let response = client
        .post("/users")
        .header(ContentType::JSON)
        .body(user_data.to_string())
        .dispatch()
        .await;

    cleanup_test_user(&pool, "test_username", "test_email@example.com").await;

    assert_eq!(response.status(), Status::Ok);

    let response_body = response.into_string().await.expect("Response has a body");
    let user_out: UserOutDTO = serde_json::from_str(&response_body).expect("Valid UserOutDTO");

    assert_eq!(user_out.username, "test_username");
    assert_eq!(user_out.email, "test_email@example.com");
}

#[rocket::async_test]
async fn create_user_integration_test_type_two() {
    let (_client, pool) = setup().await;

    let user_dto = UserInDTO {
        username: "testuserpost".to_string(),
        email: "testuserpost@example.com".to_string(),
    };

    let response = create_user(&pool, &user_dto).await;

    assert!(response.is_ok());

    let user = response.unwrap();

    cleanup_test_user(&pool, "testuserpost", "testuserpost@example.com").await;

    assert_eq!(user.username, "testuserpost");
    assert_eq!(user.email, "testuserpost@example.com");
}

#[rocket::async_test]
async fn get_all_users_integration_test() {
    let (client, _pool) = setup().await;

    let response = client.get("/users").dispatch().await;

    assert_eq!(response.status(), Status::Ok);

    let response_body = response.into_string().await.expect("Response has a body");
    let users: Vec<UserOutDTO> =
        serde_json::from_str(&response_body).expect("Valid list of UserOutDTO");

    assert!(users.len() > 0);
}

#[rocket::async_test]
async fn get_user_by_id_integration_test() {
    let (client, pool) = setup().await;

    let user_dto = UserInDTO {
        username: "testuserid".to_string(),
        email: "testuserid@example.com".to_string(),
    };

    let response = create_user(&pool, &user_dto).await;
    let user = response.unwrap();

    let user_id = user.id;

    let response = client.get(format!("/users/{}", user_id)).dispatch().await;

    assert_eq!(response.status(), Status::Ok);

    let response_body = response.into_string().await.expect("Response has a body");
    let user: UserOutDTO = serde_json::from_str(&response_body).expect("Valid UserOutDTO");

    cleanup_test_user(&pool, "testuserid", "testuserid@example.com").await;

    assert_eq!(user.id, user_id);
}

#[rocket::async_test]
async fn update_user_integration_test() {
    let (client, pool) = setup().await;

    let user_data = json!({
        "username": "update_test_username",
        "email": "update_test_email@example.com"
    });
    let create_response = client
        .post("/users")
        .header(ContentType::JSON)
        .body(user_data.to_string())
        .dispatch()
        .await;
    let created_user: UserOutDTO = serde_json::from_str(
        &create_response
            .into_string()
            .await
            .expect("Response has a body"),
    )
    .expect("Valid UserOutDTO");

    let update_data = json!({
        "username": "updated_username",
        "email": "updated_email@example.com"
    });
    let response = client
        .patch(format!("/users/{}", created_user.id))
        .header(ContentType::JSON)
        .body(update_data.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let response_body = response.into_string().await.expect("Response has a body");
    let updated_user: UserOutDTO = serde_json::from_str(&response_body).expect("Valid UserOutDTO");

    assert_eq!(updated_user.username, "updated_username");
    assert_eq!(updated_user.email, "updated_email@example.com");

    cleanup_test_user(&pool, "updated_username", "updated_email@example.com").await;
}

#[rocket::async_test]
async fn delete_user_integration_test() {
    // let (client, _pool) = setup().await;

    // let user_data = json!({
    //     "username": "delete_test_username",
    //     "email": "delete_test_email@example.com"
    // });
    // let create_response = client
    //     .post("/users")
    //     .header(ContentType::JSON)
    //     .body(user_data.to_string())
    //     .dispatch()
    //     .await;
    // let created_user: UserOutDTO = serde_json::from_str(
    //     &create_response
    //         .into_string()
    //         .await
    //         .expect("Response has a body"),
    // )
    // .expect("Valid UserOutDTO");

    let (client, pool) = setup().await;

    let user_dto = UserInDTO {
        username: "testuser".to_string(),
        email: "testuser@example.com".to_string(),
    };

    let response_saved = create_user(&pool, &user_dto).await;

    let user = response_saved.unwrap();

    let response = client
        .delete(format!("/users/{}", user.id))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::NoContent);

    let fetch_response = client.get(format!("/users/{}", user.id)).dispatch().await;
    assert_eq!(fetch_response.status(), Status::NotFound);
}
