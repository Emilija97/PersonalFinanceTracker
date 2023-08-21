use personal_finance_tracker::dtos::account_dtos::{AccountInDTO, AccountOutDTO};
use personal_finance_tracker::enums::custom_enums::AccountType;
use personal_finance_tracker::models::user::User;
use personal_finance_tracker::operations::account_ops::create_account;
use personal_finance_tracker::operations::user_ops::create_user;
use rocket::http::{ContentType, Status};
use rocket::serde::json::serde_json;
use serde_json::json;

use personal_finance_tracker::dtos::user_dtos::UserInDTO;
use sqlx::{Error, PgPool};

use crate::common::setup;
use uuid::Uuid;

mod common;

async fn before_test(pool: &PgPool, username: &str, email: &str) -> Result<User, Error> {
    let user_dto = UserInDTO {
        username: username.to_string(),
        email: email.to_string(),
    };

    let user = create_user(&pool, &user_dto).await?;
    Ok(user)
}

async fn cleanup_test(pool: &PgPool, id: Uuid) {
    sqlx::query("DELETE FROM accounts WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .expect("Failed to cleanup test account");
}

async fn cleanup_test_user(pool: &PgPool, username: &str, email: &str) {
    sqlx::query("DELETE FROM users WHERE username = $1 AND email = $2")
        .bind(username)
        .bind(email)
        .execute(pool)
        .await
        .expect("Failed to cleanup test user");
}

#[rocket::async_test]
async fn create_account_integration_test() {
    let (_client, pool) = setup().await;

    let user = before_test(&pool, "createuser", "createuser@example.com")
        .await
        .expect("Failed to initialize test database");
    println!("Created test user: {:?}", user);

    let account_dto = AccountInDTO {
        name: "Test".to_string(),
        balance: 100.0,
        account_type: AccountType::Bank,
        user_id: user.id,
    };

    let account = create_account(&pool, &account_dto)
        .await
        .expect("Failed to create account");

    // assert!(response.is_ok());
    // let account = response.unwrap();
    assert_eq!(account.name, "Test");

    cleanup_test_user(&pool, "createuser", "createuser@example.com").await;
    cleanup_test(&pool, account.id).await;
}

#[rocket::async_test]
async fn get_all_accounts_test() {
    let (client, _pool) = setup().await;
    let response = client.get("/accounts").dispatch().await;

    assert_eq!(response.status(), Status::Ok);
}

#[rocket::async_test]
async fn get_account_by_id_integration_test() {
    let (client, pool) = setup().await;

    match before_test(&pool, "testuserid", "testuserid@example.com").await {
        Ok(user) => {
            let account_dto = AccountInDTO {
                name: "Test".to_string(),
                balance: 100.0,
                account_type: AccountType::Bank,
                user_id: user.id,
            };

            match create_account(&pool, &account_dto).await {
                Ok(account) => {
                    assert_eq!(account.name, "Test");

                    let response = client
                        .get(format!("/accounts/{}", account.id))
                        .dispatch()
                        .await;

                    assert_eq!(response.status(), Status::Ok);

                    if let Some(response_body) = response.into_string().await {
                        match serde_json::from_str::<AccountOutDTO>(&response_body) {
                            Ok(founded_account) => {
                                cleanup_test_user(&pool, "testuserid", "testuserid@example.com")
                                    .await;
                                cleanup_test(&pool, account.id).await;
                                assert_eq!(account.name, founded_account.name.to_string());
                            }
                            Err(_) => assert!(false, "Failed to deserialize AccountOutDTO"),
                        }
                    } else {
                        assert!(false, "Response does not have a body");
                    }
                }
                Err(_) => assert!(false, "Failed to create account"),
            }
        }
        Err(_) => assert!(false, "Failed to initialize test database"),
    }
}

#[rocket::async_test]
async fn update_account_integration_test() {
    let (client, pool) = setup().await;

    let user = before_test(&pool, "updateuser", "updateuser@example.com")
        .await
        .expect("Failed to initialize test database");

    let account_data = json!({
        "name": "Test".to_string(),
        "balance": 100.0,
        "account_type": AccountType::Bank,
        "user_id": user.id,
    });
    let create_response = client
        .post("/accounts")
        .header(ContentType::JSON)
        .body(account_data.to_string())
        .dispatch()
        .await;
    let created_account: AccountOutDTO = serde_json::from_str(
        &create_response
            .into_string()
            .await
            .expect("Response has a body"),
    )
    .expect("Valid AccountOutDTO");

    let update_data = json!({
        "name": "Test updated",
        "balance": 150.0,
        "account_type": AccountType::Bank,
        "user_id": user.id
    });
    let response = client
        .patch(format!("/accounts/{}", created_account.id))
        .header(ContentType::JSON)
        .body(update_data.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let response_body = response.into_string().await.expect("Response has a body");
    let updated_account: AccountOutDTO =
        serde_json::from_str(&response_body).expect("Valid AccountOutDTO");

    assert_eq!(updated_account.name, "Test updated");
    assert_eq!(updated_account.balance, 150.0);

    cleanup_test_user(&pool, "updateuser", "updateuser@example.com").await;
    cleanup_test(&pool, updated_account.id).await;
}

#[rocket::async_test]
async fn delete_account_integration_test() {
    let (client, pool) = setup().await;

    let user = before_test(&pool, "testuser", "testuser@example.com")
        .await
        .expect("Failed to initialize test database");

    let account_dto = AccountInDTO {
        name: "Test".to_string(),
        balance: 100.0,
        account_type: AccountType::Bank,
        user_id: user.id,
    };

    let account = create_account(&pool, &account_dto)
        .await
        .expect("Failed to create account");
    assert_eq!(account.name, "Test");

    let response = client
        .delete(format!("/accounts/{}", account.id))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::NoContent);

    let fetch_response = client
        .get(format!("/accounts/{}", account.id))
        .dispatch()
        .await;
    assert_eq!(fetch_response.status(), Status::NotFound);
    cleanup_test_user(&pool, "testuser", "testuser@example.com").await;
}
