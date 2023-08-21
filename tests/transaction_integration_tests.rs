use chrono::Local;
use personal_finance_tracker::dtos::account_dtos::AccountInDTO;
use personal_finance_tracker::dtos::category_dtos::CategoryInDTO;
use personal_finance_tracker::dtos::transaction_dtos::{TransactionInDTO, TransactionOutDTO};
use personal_finance_tracker::enums::custom_enums::{AccountType, TransactionType};
use personal_finance_tracker::operations::account_ops::create_account;
use personal_finance_tracker::operations::category_ops::create_category;
use personal_finance_tracker::operations::transaction_ops::create_transaction;
use personal_finance_tracker::operations::user_ops::create_user;
use rocket::http::{ContentType, Status};
use rocket::serde::json::serde_json;
use serde_json::json;

use personal_finance_tracker::dtos::user_dtos::UserInDTO;
use sqlx::PgPool;

use crate::common::setup;
use uuid::Uuid;

mod common;

pub async fn before_test(
    pool: &PgPool,
    username: &str,
    email: &str,
) -> Result<(Uuid, Uuid, Uuid), sqlx::Error> {
    // Create a user
    let user_dto = UserInDTO {
        username: username.to_string(),
        email: email.to_string(),
    };
    let user = create_user(&pool, &user_dto).await?;

    // Create an account
    let account_dto = AccountInDTO {
        name: "Test".to_string(),
        balance: 100.0,
        account_type: AccountType::Bank,
        user_id: user.id,
    };
    let account = create_account(&pool, &account_dto).await?;

    // Create a category
    let category_dto = CategoryInDTO {
        name: "TestCategory".to_string(),
        user_id: user.id,
    };
    let category = create_category(&pool, &category_dto).await?;

    Ok((user.id, account.id, category.id))
}

pub async fn cleanup(
    pool: &PgPool,
    user_id: Uuid,
    account_id: Uuid,
    category_id: Uuid,
    transaction_id: Option<Uuid>,
) {
    let mut tables = vec![
        ("users", user_id),
        ("accounts", account_id),
        ("categories", category_id),
    ];

    if let Some(tx_id) = transaction_id {
        tables.push(("transactions", tx_id));
    }

    for (table, id) in tables.iter() {
        sqlx::query(&format!("DELETE FROM {} WHERE id = $1", table))
            .bind(id)
            .execute(pool)
            .await
            .unwrap_or_else(|e| panic!("Failed to cleanup test {}: {}", table, e));
    }
}

#[rocket::async_test]
async fn create_transaction_integration_test() {
    let (_client, pool) = setup().await;

    let (user_id, account_id, category_id) =
        before_test(&pool, "createuser", "createuser@example.com")
            .await
            .expect("Failed to initialize test database");

    let transaction_dto = TransactionInDTO {
        title: "Test transaction".to_string(),
        amount: 100.0,
        transaction_type: TransactionType::Income,
        user_id: user_id,
        date: Local::now().naive_local(),
        category_id,
        account_id: account_id,
    };

    let transaction = create_transaction(&pool, &transaction_dto)
        .await
        .expect("Failed to create transaction");

    // assert!(response.is_ok());
    // let transaction = response.unwrap();
    assert_eq!(transaction.title, "Test transaction");

    cleanup(
        &pool,
        user_id,
        account_id,
        category_id,
        Some(transaction.id),
    )
    .await;
}

#[rocket::async_test]
async fn get_all_transactions_test() {
    let (client, _pool) = setup().await;
    let response = client.get("/transactions").dispatch().await;

    assert_eq!(response.status(), Status::Ok);
}

#[rocket::async_test]
async fn get_transaction_by_id_integration_test() {
    let (client, pool) = setup().await;

    match before_test(&pool, "testuserid", "testuserid@example.com").await {
        Ok((user_id, account_id, category_id)) => {
            let transaction_dto = TransactionInDTO {
                title: "Test transaction".to_string(),
                amount: 100.0,
                transaction_type: TransactionType::Income,
                user_id: user_id,
                date: Local::now().naive_local(),
                category_id,
                account_id: account_id,
            };

            match create_transaction(&pool, &transaction_dto).await {
                Ok(transaction) => {
                    assert_eq!(transaction.title, "Test transaction");

                    let response = client
                        .get(format!("/transactions/{}", transaction.id))
                        .dispatch()
                        .await;

                    assert_eq!(response.status(), Status::Ok);

                    if let Some(response_body) = response.into_string().await {
                        match serde_json::from_str::<TransactionOutDTO>(&response_body) {
                            Ok(founded_transaction) => {
                                cleanup(
                                    &pool,
                                    user_id,
                                    account_id,
                                    category_id,
                                    Some(transaction.id),
                                )
                                .await;
                                assert_eq!(
                                    transaction.title,
                                    founded_transaction.title.to_string()
                                );
                            }
                            Err(_) => assert!(false, "Failed to deserialize TransactionOutDTO"),
                        }
                    } else {
                        assert!(false, "Response does not have a body");
                    }
                }
                Err(_) => assert!(false, "Failed to create transaction"),
            }
        }
        Err(_) => assert!(false, "Failed to initialize test database"),
    }
}

#[rocket::async_test]
async fn update_transaction_integration_test() {
    let (client, pool) = setup().await;

    let (user_id, account_id, category_id) =
        before_test(&pool, "updateuser", "updateuser@example.com")
            .await
            .expect("Failed to initialize test database");

    let transaction_data = json!({
        "title": "Test transaction".to_string(),
        "amount": 100.0,
        "transaction_type": TransactionType::Income,
        "user_id": user_id,
        "date": Local::now().naive_local(),
        "category_id": category_id,
        "account_id": account_id,
    });
    let create_response = client
        .post("/transactions")
        .header(ContentType::JSON)
        .body(transaction_data.to_string())
        .dispatch()
        .await;
    let created_transaction: TransactionOutDTO = serde_json::from_str(
        &create_response
            .into_string()
            .await
            .expect("Response has a body"),
    )
    .expect("Valid TransactionOutDTO");

    println!("Created transaction {}", created_transaction.title);

    let update_data = json!({
        "title": "Test transaction updated".to_string(),
        "amount": 150.0,
        "transaction_type": TransactionType::Income,
        "user_id": user_id,
        "date": Local::now().naive_local(),
        "category_id": category_id,
        "account_id": account_id,
    });
    let response = client
        .patch(format!("/transactions/{}", created_transaction.id))
        .header(ContentType::JSON)
        .body(update_data.to_string())
        .dispatch()
        .await;
    print!("{}", response.status());

    assert_eq!(response.status(), Status::Ok);

    let response_body = response.into_string().await.expect("Response has a body");
    let updated_transaction: TransactionOutDTO =
        serde_json::from_str(&response_body).expect("Valid TransactionOutDTO");

    assert_eq!(updated_transaction.title, "Test transaction updated");
    assert_eq!(updated_transaction.amount, 150.0);

    cleanup(
        &pool,
        user_id,
        account_id,
        category_id,
        Some(updated_transaction.id),
    )
    .await;
}

#[rocket::async_test]
async fn delete_transaction_integration_test() {
    let (client, pool) = setup().await;

    let (user_id, account_id, category_id) = before_test(&pool, "testuser", "testuser@example.com")
        .await
        .expect("Failed to initialize test database");

    let transaction_dto = TransactionInDTO {
        title: "Test transaction".to_string(),
        amount: 100.0,
        transaction_type: TransactionType::Income,
        user_id: user_id,
        date: Local::now().naive_local(),
        category_id,
        account_id: account_id,
    };

    let transaction = create_transaction(&pool, &transaction_dto)
        .await
        .expect("Failed to create transaction");
    assert_eq!(transaction.title, "Test transaction");

    let response = client
        .delete(format!("/transactions/{}", transaction.id))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::NoContent);

    let fetch_response = client
        .get(format!("/transactions/{}", transaction.id))
        .dispatch()
        .await;
    assert_eq!(fetch_response.status(), Status::NotFound);
    cleanup(&pool, user_id, account_id, category_id, None).await;
}
