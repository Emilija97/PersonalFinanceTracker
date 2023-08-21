use personal_finance_tracker::dtos::user_dtos::UserInDTO;
use personal_finance_tracker::operations::category_ops::create_category;
use personal_finance_tracker::operations::user_ops::create_user;
use rocket::http::{ContentType, Status};
use rocket::serde::json::serde_json;
use serde_json::json;

use personal_finance_tracker::dtos::category_dtos::{CategoryInDTO, CategoryOutDTO};
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

pub async fn cleanup(pool: &PgPool, user_id: Uuid, category_id: Option<Uuid>) {
    let mut tables = Vec::new();

    tables.push(("users", user_id));

    if let Some(cat_id) = category_id {
        tables.push(("categories", cat_id));
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
async fn create_category_integration_test() {
    let (client, pool) = setup().await;

    let user_id = before_test(&pool, "createuser", "createuser@example.com")
        .await
        .expect("Failed to initialize test database");

    let category_data = json!({
        "name": "Groceries",
        "user_id": user_id
    });

    let response = client
        .post("/categories")
        .header(ContentType::JSON)
        .body(category_data.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let response_body = response.into_string().await.expect("Response has a body");
    let category_out: CategoryOutDTO =
        serde_json::from_str(&response_body).expect("Valid CategoryOutDTO");

    assert_eq!(category_out.name, "Groceries");
    cleanup(&pool, user_id, Some(category_out.id)).await;
}

#[rocket::async_test]
async fn create_category_integration_test_type_two() {
    let (_client, pool) = setup().await;

    let user_id = before_test(&pool, "createuser2", "createuser2@example.com")
        .await
        .expect("Failed to initialize test database");

    let category_dto = CategoryInDTO {
        name: "Groceries".to_string(),
        user_id: user_id,
    };

    let response = create_category(&pool, &category_dto).await;

    assert!(response.is_ok());

    let category = response.unwrap();

    assert_eq!(category.name, "Groceries");
    assert_eq!(category.user_id, user_id);

    cleanup(&pool, user_id, Some(category.id)).await;
}

#[rocket::async_test]
async fn get_all_categories_integration_test() {
    let (client, _pool) = setup().await;

    let response = client.get("/categories").dispatch().await;

    assert_eq!(response.status(), Status::Ok);

    // let response_body = response.into_string().await.expect("Response has a body");
    // let categories: Vec<CategoryOutDTO> =
    //     serde_json::from_str(&response_body).expect("Valid list of CategoryOutDTO");

    // assert!(categories.len() > 0);
}

#[rocket::async_test]
async fn get_category_by_id_integration_test() {
    let (client, pool) = setup().await;

    let user_id = before_test(&pool, "testuserid", "testuserid@example.com")
        .await
        .expect("Failed to initialize test database");

    let category_dto = CategoryInDTO {
        name: "Groceries".to_string(),
        user_id: user_id,
    };

    let response = create_category(&pool, &category_dto).await;
    let category = response.unwrap();

    let category_id = category.id;

    let response = client
        .get(format!("/categories/{}", category_id))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let response_body = response.into_string().await.expect("Response has a body");
    let category: CategoryOutDTO =
        serde_json::from_str(&response_body).expect("Valid CategoryOutDTO");
    assert_eq!(category.id, category_id);

    cleanup(&pool, user_id, Some(category.id)).await;
}

#[rocket::async_test]
async fn update_category_integration_test() {
    let (client, pool) = setup().await;

    let user_id = before_test(&pool, "updateuser", "updateuser@example.com")
        .await
        .expect("Failed to initialize test database");

    let category_data = json!({
        "name": "Groceries",
        "user_id": user_id
    });
    let create_response = client
        .post("/categories")
        .header(ContentType::JSON)
        .body(category_data.to_string())
        .dispatch()
        .await;
    let created_category: CategoryOutDTO = serde_json::from_str(
        &create_response
            .into_string()
            .await
            .expect("Response has a body"),
    )
    .expect("Valid CategoryOutDTO");

    let update_data = json!({
        "name": "Groceries updated",
        "user_id": user_id
    });
    let response = client
        .patch(format!("/categories/{}", created_category.id))
        .header(ContentType::JSON)
        .body(update_data.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let response_body = response.into_string().await.expect("Response has a body");
    let updated_category: CategoryOutDTO =
        serde_json::from_str(&response_body).expect("Valid CategoryOutDTO");

    assert_eq!(updated_category.name, "Groceries updated");
    assert_eq!(updated_category.user_id, user_id);

    cleanup(&pool, user_id, Some(updated_category.id)).await;
}

#[rocket::async_test]
async fn delete_category_integration_test() {
    let (client, pool) = setup().await;

    let user_id = before_test(&pool, "deleteuser", "deleteuser@example.com")
        .await
        .expect("Failed to initialize test database");

    let category_dto = CategoryInDTO {
        name: "Groceries".to_string(),
        user_id: user_id,
    };

    let response_saved = create_category(&pool, &category_dto).await;

    let category = response_saved.unwrap();

    let response = client
        .delete(format!("/categories/{}", category.id))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::NoContent);

    let fetch_response = client
        .get(format!("/categories/{}", category.id))
        .dispatch()
        .await;
    assert_eq!(fetch_response.status(), Status::NotFound);

    cleanup(&pool, user_id, None).await;
}
