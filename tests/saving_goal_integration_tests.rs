use chrono::{Local, NaiveDate};
use personal_finance_tracker::dtos::user_dtos::UserInDTO;
use personal_finance_tracker::operations::saving_goal_ops::create_saving_goal;
use personal_finance_tracker::operations::user_ops::create_user;
use rocket::http::{ContentType, Status};
use rocket::serde::json::serde_json;
use serde_json::json;

use personal_finance_tracker::dtos::saving_goal_dtos::{SavingGoalInDTO, SavingGoalOutDTO};
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

pub async fn cleanup(pool: &PgPool, user_id: Uuid, saving_goal_id: Option<Uuid>) {
    let mut tables = Vec::new();

    tables.push(("users", user_id));

    if let Some(cat_id) = saving_goal_id {
        tables.push(("saving_goals", cat_id));
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
async fn create_saving_goal_integration_test() {
    let (client, pool) = setup().await;

    let user_id = before_test(&pool, "createuser", "createuser@example.com")
        .await
        .expect("Failed to initialize test database");

    let saving_goal_data = json!({
        "title": "Vacation",
        "target_amount": 2000.0,
        "current_amount": 100.0,
        "deadline": NaiveDate::from_ymd_opt(2024, 12, 31),
        "user_id": user_id,
    });

    let response = client
        .post("/saving_goals")
        .header(ContentType::JSON)
        .body(saving_goal_data.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let response_body = response.into_string().await.expect("Response has a body");
    let saving_goal_out: SavingGoalOutDTO =
        serde_json::from_str(&response_body).expect("Valid SavingGoalOutDTO");

    assert_eq!(saving_goal_out.title, "Vacation");
    cleanup(&pool, user_id, Some(saving_goal_out.id)).await;
}

#[rocket::async_test]
async fn create_saving_goal_integration_test_type_two() {
    let (_client, pool) = setup().await;

    let user_id = before_test(&pool, "createuser2", "createuser2@example.com")
        .await
        .expect("Failed to initialize test database");

    let deadline_option = chrono::NaiveDate::from_ymd_opt(2024, 12, 31);

    let deadline = deadline_option.expect("Invalid date");

    let saving_goal_dto = SavingGoalInDTO {
        title: "Wedding".to_string(),
        target_amount: 1000.0,
        current_amount: 100.0,
        deadline,
        user_id: user_id,
    };

    let response = create_saving_goal(&pool, &saving_goal_dto).await;

    assert!(response.is_ok());

    let saving_goal = response.unwrap();

    assert_eq!(saving_goal.title, "Wedding");
    assert_eq!(saving_goal.user_id, user_id);

    cleanup(&pool, user_id, Some(saving_goal.id)).await;
}

#[rocket::async_test]
async fn get_all_saving_goals_integration_test() {
    let (client, _pool) = setup().await;

    let response = client.get("/saving_goals").dispatch().await;

    assert_eq!(response.status(), Status::Ok);
}

#[rocket::async_test]
async fn get_saving_goal_by_id_integration_test() {
    let (client, pool) = setup().await;

    let user_id = before_test(&pool, "testuserid", "testuserid@example.com")
        .await
        .expect("Failed to initialize test database");

    let deadline_option = chrono::NaiveDate::from_ymd_opt(2024, 12, 31);

    let deadline = deadline_option.expect("Invalid date");

    let saving_goal_dto = SavingGoalInDTO {
        title: "Wedding".to_string(),
        target_amount: 1000.0,
        current_amount: 100.0,
        deadline: deadline,
        user_id: user_id,
    };

    let response = create_saving_goal(&pool, &saving_goal_dto).await;
    let saving_goal = response.unwrap();

    let saving_goal_id = saving_goal.id;

    let response = client
        .get(format!("/saving_goals/{}", saving_goal_id))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let response_body = response.into_string().await.expect("Response has a body");
    let saving_goal: SavingGoalOutDTO =
        serde_json::from_str(&response_body).expect("Valid SavingGoalOutDTO");
    assert_eq!(saving_goal.id, saving_goal_id);

    cleanup(&pool, user_id, Some(saving_goal.id)).await;
}

#[rocket::async_test]
async fn update_saving_goal_integration_test() {
    let (client, pool) = setup().await;

    let user_id = before_test(&pool, "updateuser", "updateuser@example.com")
        .await
        .expect("Failed to initialize test database");

    let saving_goal_data = json!({
        "title": "Vacation",
        "target_amount": 2000.0,
        "current_amount": 100.0,
        "deadline": NaiveDate::from_ymd_opt(2024, 12, 31),
        "user_id": user_id,
    });
    let create_response = client
        .post("/saving_goals")
        .header(ContentType::JSON)
        .body(saving_goal_data.to_string())
        .dispatch()
        .await;
    let created_saving_goal: SavingGoalOutDTO = serde_json::from_str(
        &create_response
            .into_string()
            .await
            .expect("Response has a body"),
    )
    .expect("Valid SavingGoalOutDTO");

    let update_data = json!({
        "title": "Vacation updated",
        "target_amount": 2000.0,
        "current_amount": 300.0,
        "deadline": NaiveDate::from_ymd_opt(2024, 12, 31),
        "user_id": user_id,
        "updated_at": Local::now().naive_local()
    });
    let response = client
        .patch(format!("/saving_goals/{}", created_saving_goal.id))
        .header(ContentType::JSON)
        .body(update_data.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let response_body = response.into_string().await.expect("Response has a body");
    let updated_saving_goal: SavingGoalOutDTO =
        serde_json::from_str(&response_body).expect("Valid SavingGoalOutDTO");

    assert_eq!(updated_saving_goal.title, "Vacation updated");
    assert_eq!(updated_saving_goal.user_id, user_id);
    assert_eq!(updated_saving_goal.current_amount, 300.0);

    cleanup(&pool, user_id, Some(updated_saving_goal.id)).await;
}

#[rocket::async_test]
async fn delete_saving_goal_integration_test() {
    let (client, pool) = setup().await;

    let user_id = before_test(&pool, "deleteuser", "deleteuser@example.com")
        .await
        .expect("Failed to initialize test database");

    let deadline_option = chrono::NaiveDate::from_ymd_opt(2024, 12, 31);

    let deadline = deadline_option.expect("Invalid date");

    let saving_goal_dto = SavingGoalInDTO {
        title: "Wedding".to_string(),
        target_amount: 1000.0,
        current_amount: 100.0,
        deadline: deadline,
        user_id: user_id,
    };

    let response_saved = create_saving_goal(&pool, &saving_goal_dto).await;

    let saving_goal = response_saved.unwrap();

    let response = client
        .delete(format!("/saving_goals/{}", saving_goal.id))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::NoContent);

    let fetch_response = client
        .get(format!("/saving_goals/{}", saving_goal.id))
        .dispatch()
        .await;
    assert_eq!(fetch_response.status(), Status::NotFound);

    cleanup(&pool, user_id, None).await;
}
