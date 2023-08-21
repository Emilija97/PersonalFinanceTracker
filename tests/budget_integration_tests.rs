use chrono::Local;
use personal_finance_tracker::dtos::user_dtos::UserInDTO;
use personal_finance_tracker::operations::budget_ops::create_budget;
use personal_finance_tracker::operations::user_ops::create_user;
use rocket::http::{ContentType, Status};
use rocket::serde::json::serde_json;
use serde_json::json;

use personal_finance_tracker::dtos::budget_dtos::{BudgetInDTO, BudgetOutDTO};
use sqlx::PgPool;
use uuid::Uuid;

use crate::common::setup;

mod common;

pub async fn before_test(pool: &PgPool, username: &str, email: &str) -> Result<Uuid, sqlx::Error> {
    // Create a user
    let user_dto = UserInDTO {
        username: username.to_string(),
        email: email.to_string(),
    };
    let user = create_user(&pool, &user_dto).await?;

    Ok(user.id)
}

pub async fn cleanup(pool: &PgPool, user_id: Uuid, budget_id: Option<Uuid>) {
    let mut tables = Vec::new();

    tables.push(("users", user_id));

    if let Some(cat_id) = budget_id {
        tables.push(("budgets", cat_id));
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
async fn create_budget_integration_test() {
    let (client, pool) = setup().await;

    let user_id = before_test(&pool, "createuser", "createuser@example.com")
        .await
        .expect("Failed to initialize test database");

    let budget_data = json!({
        "name": "Groceries".to_string(),
        "user_id": user_id,
        "amount": 100.0,
        "start_date": Local::now().naive_local(),
        "end_date": Local::now().naive_local(),
    });

    let response = client
        .post("/budgets")
        .header(ContentType::JSON)
        .body(budget_data.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let response_body = response.into_string().await.expect("Response has a body");
    let budget_out: BudgetOutDTO =
        serde_json::from_str(&response_body).expect("Valid BudgetOutDTO");

    assert_eq!(budget_out.name, "Groceries");
    cleanup(&pool, user_id, Some(budget_out.id)).await;
}

#[rocket::async_test]
async fn create_budget_integration_test_type_two() {
    let (_client, pool) = setup().await;

    let user_id = before_test(&pool, "createuser2", "createuser2@example.com")
        .await
        .expect("Failed to initialize test database");

    let budget_dto = BudgetInDTO {
        name: "Groceries".to_string(),
        user_id: user_id,
        amount: 100.0,
        start_date: Local::now().naive_local(),
        end_date: Local::now().naive_local(),
        category_id: None,
    };

    let response = create_budget(&pool, &budget_dto).await;

    assert!(response.is_ok());

    let budget = response.unwrap();

    assert_eq!(budget.name, "Groceries");
    assert_eq!(budget.user_id, user_id);

    cleanup(&pool, user_id, Some(budget.id)).await;
}

#[rocket::async_test]
async fn get_all_budgets_integration_test() {
    let (client, _pool) = setup().await;

    let response = client.get("/budgets").dispatch().await;

    assert_eq!(response.status(), Status::Ok);

    // let response_body = response.into_string().await.expect("Response has a body");
    // let budgets: Vec<BudgetOutDTO> =
    //     serde_json::from_str(&response_body).expect("Valid list of BudgetOutDTO");

    // assert!(budgets.len() > 0);
}

#[rocket::async_test]
async fn get_budget_by_id_integration_test() {
    let (client, pool) = setup().await;

    let user_id = before_test(&pool, "testuserid", "testuserid@example.com")
        .await
        .expect("Failed to initialize test database");

    let budget_dto = BudgetInDTO {
        name: "Groceries".to_string(),
        user_id: user_id,
        amount: 100.0,
        start_date: Local::now().naive_local(),
        end_date: Local::now().naive_local(),
        category_id: None,
    };

    let response = create_budget(&pool, &budget_dto).await;
    let budget = response.unwrap();

    let budget_id = budget.id;

    let response = client
        .get(format!("/budgets/{}", budget_id))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let response_body = response.into_string().await.expect("Response has a body");
    let budget: BudgetOutDTO = serde_json::from_str(&response_body).expect("Valid BudgetOutDTO");
    assert_eq!(budget.id, budget_id);

    cleanup(&pool, user_id, Some(budget.id)).await;
}

#[rocket::async_test]
async fn update_budget_integration_test() {
    let (client, pool) = setup().await;

    let user_id = before_test(&pool, "updateuser", "updateuser@example.com")
        .await
        .expect("Failed to initialize test database");

    let budget_data = json!({
        "name": "Groceries",
        "user_id": user_id,
        "amount": 100.0,
        "start_date": Local::now().naive_local(),
        "end_date": Local::now().naive_local(),
    });
    let create_response = client
        .post("/budgets")
        .header(ContentType::JSON)
        .body(budget_data.to_string())
        .dispatch()
        .await;
    let created_budget: BudgetOutDTO = serde_json::from_str(
        &create_response
            .into_string()
            .await
            .expect("Response has a body"),
    )
    .expect("Valid BudgetOutDTO");

    let update_data = json!({
        "name": "Groceries updated",
        "user_id": user_id,
        "amount": 150.0,
        "start_date": Local::now().naive_local(),
        "end_date": Local::now().naive_local(),
    });
    let response = client
        .patch(format!("/budgets/{}", created_budget.id))
        .header(ContentType::JSON)
        .body(update_data.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let response_body = response.into_string().await.expect("Response has a body");
    let updated_budget: BudgetOutDTO =
        serde_json::from_str(&response_body).expect("Valid BudgetOutDTO");

    assert_eq!(updated_budget.name, "Groceries updated");
    assert_eq!(updated_budget.user_id, user_id);
    assert_eq!(updated_budget.amount, 150.0);

    cleanup(&pool, user_id, Some(updated_budget.id)).await;
}

#[rocket::async_test]
async fn delete_budget_integration_test() {
    let (client, pool) = setup().await;

    let user_id = before_test(&pool, "deleteuser", "deleteuser@example.com")
        .await
        .expect("Failed to initialize test database");

    let budget_dto = BudgetInDTO {
        name: "Groceries".to_string(),
        user_id: user_id,
        amount: 100.0,
        start_date: Local::now().naive_local(),
        end_date: Local::now().naive_local(),
        category_id: None,
    };

    let response_saved = create_budget(&pool, &budget_dto).await;

    let budget = response_saved.unwrap();

    let response = client
        .delete(format!("/budgets/{}", budget.id))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::NoContent);

    let fetch_response = client
        .get(format!("/budgets/{}", budget.id))
        .dispatch()
        .await;
    assert_eq!(fetch_response.status(), Status::NotFound);

    cleanup(&pool, user_id, None).await;
}
