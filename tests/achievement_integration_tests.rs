use chrono::Local;
use personal_finance_tracker::dtos::saving_goal_dtos::SavingGoalInDTO;
use personal_finance_tracker::dtos::user_dtos::UserInDTO;
use personal_finance_tracker::operations::achievement_ops::create_achievement;
use personal_finance_tracker::operations::saving_goal_ops::create_saving_goal;
use personal_finance_tracker::operations::user_ops::create_user;
use rocket::http::{ContentType, Status};
use rocket::serde::json::serde_json;
use serde_json::json;

use personal_finance_tracker::dtos::achievement_dtos::{AchievementInDTO, AchievementOutDTO};
use sqlx::PgPool;
use uuid::Uuid;

use crate::common::setup;

mod common;

pub async fn before_test(
    pool: &PgPool,
    username: &str,
    email: &str,
) -> Result<(Uuid, Uuid), sqlx::Error> {
    // Create a user
    let user_dto = UserInDTO {
        username: username.to_string(),
        email: email.to_string(),
    };
    let user = create_user(&pool, &user_dto).await?;

    let deadline_option = chrono::NaiveDate::from_ymd_opt(2024, 12, 31);

    let deadline = deadline_option.expect("Invalid date");

    //Create a saving goal
    let saving_goal_dto = SavingGoalInDTO {
        title: "Vacation".to_string(),
        target_amount: 2000.0,
        current_amount: 100.0,
        deadline,
        user_id: user.id,
    };

    let saving_goal = create_saving_goal(&pool, &saving_goal_dto).await?;

    Ok((user.id, saving_goal.id))
}

pub async fn cleanup(
    pool: &PgPool,
    user_id: Uuid,
    saving_goal_id: Uuid,
    achievement_id: Option<Uuid>,
) {
    let mut tables = vec![("users", user_id), ("saving_goals", saving_goal_id)];

    if let Some(ach_id) = achievement_id {
        tables.push(("achievements", ach_id));
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
async fn create_achievement_integration_test() {
    let (client, pool) = setup().await;

    let (user_id, saving_goal_id) = before_test(&pool, "createuser", "createuser@example.com")
        .await
        .expect("Failed to initialize test database");

    let achievement_data = json!({
        "goal_id": saving_goal_id,
        "amount_saved": 1000.0,
        "date_achieved": Local::now().naive_local(),
    });

    let response = client
        .post("/achievements")
        .header(ContentType::JSON)
        .body(achievement_data.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let response_body = response.into_string().await.expect("Response has a body");
    let achievement_out: AchievementOutDTO =
        serde_json::from_str(&response_body).expect("Valid AchievementOutDTO");

    assert_eq!(achievement_out.goal_id, saving_goal_id);
    cleanup(&pool, user_id, saving_goal_id, Some(achievement_out.id)).await;
}

#[rocket::async_test]
async fn create_achievement_integration_test_type_two() {
    let (_client, pool) = setup().await;

    let (user_id, saving_goal_id) = before_test(&pool, "createuser2", "createuser2@example.com")
        .await
        .expect("Failed to initialize test database");

    let achievement_dto = AchievementInDTO {
        goal_id: saving_goal_id,
        date_achieved: Local::now().naive_local(),
        amount_saved: 1200.0,
    };

    let response = create_achievement(&pool, &achievement_dto).await;

    assert!(response.is_ok());

    let achievement = response.unwrap();

    assert_eq!(achievement.amount_saved, 1200.0);
    assert_eq!(achievement.goal_id, saving_goal_id);

    cleanup(&pool, user_id, saving_goal_id, Some(achievement.id)).await;
}

#[rocket::async_test]
async fn get_all_achievements_integration_test() {
    let (client, _pool) = setup().await;

    let response = client.get("/achievements").dispatch().await;

    assert_eq!(response.status(), Status::Ok);
}

#[rocket::async_test]
async fn get_achievement_by_id_integration_test() {
    let (client, pool) = setup().await;

    let (user_id, saving_goal_id) = before_test(&pool, "testuserid", "testuserid@example.com")
        .await
        .expect("Failed to initialize test database");

    let achievement_dto = AchievementInDTO {
        goal_id: saving_goal_id,
        date_achieved: Local::now().naive_local(),
        amount_saved: 1500.0,
    };

    let response = create_achievement(&pool, &achievement_dto).await;
    let achievement = response.unwrap();

    let achievement_id = achievement.id;

    let response = client
        .get(format!("/achievements/{}", achievement_id))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let response_body = response.into_string().await.expect("Response has a body");
    let achievement: AchievementOutDTO =
        serde_json::from_str(&response_body).expect("Valid AchievementOutDTO");
    assert_eq!(achievement.id, achievement_id);

    cleanup(&pool, user_id, saving_goal_id, Some(achievement.id)).await;
}

#[rocket::async_test]
async fn update_achievement_integration_test() {
    let (client, pool) = setup().await;

    let (user_id, saving_goal_id) = before_test(&pool, "updateuser", "updateuser@example.com")
        .await
        .expect("Failed to initialize test database");

    let first_date = Local::now().naive_local();
    let achievement_data = json!({
        "goal_id": saving_goal_id,
        "amount_saved": 1600.0,
        "date_achieved": first_date,
    });
    let create_response = client
        .post("/achievements")
        .header(ContentType::JSON)
        .body(achievement_data.to_string())
        .dispatch()
        .await;
    let created_achievement: AchievementOutDTO = serde_json::from_str(
        &create_response
            .into_string()
            .await
            .expect("Response has a body"),
    )
    .expect("Valid AchievementOutDTO");

    let updated_date = Local::now().naive_local();
    let update_data = json!({
        "goal_id": saving_goal_id,
        "amount_saved": 1700.0,
        "date_achieved": updated_date,
    });
    let response = client
        .patch(format!("/achievements/{}", created_achievement.id))
        .header(ContentType::JSON)
        .body(update_data.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let response_body = response.into_string().await.expect("Response has a body");
    let updated_achievement: AchievementOutDTO =
        serde_json::from_str(&response_body).expect("Valid AchievementOutDTO");

    assert_ne!(updated_achievement.date_achieved, first_date);
    assert_eq!(updated_achievement.amount_saved, 1700.0);

    cleanup(&pool, user_id, saving_goal_id, Some(updated_achievement.id)).await;
}

#[rocket::async_test]
async fn delete_achievement_integration_test() {
    let (client, pool) = setup().await;

    let (user_id, saving_goal_id) = before_test(&pool, "deleteuser", "deleteuser@example.com")
        .await
        .expect("Failed to initialize test database");

    let achievement_dto = AchievementInDTO {
        goal_id: saving_goal_id,
        date_achieved: Local::now().naive_local(),
        amount_saved: 1000.0,
    };

    let response_saved = create_achievement(&pool, &achievement_dto).await;

    let achievement = response_saved.unwrap();

    let response = client
        .delete(format!("/achievements/{}", achievement.id))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::NoContent);

    let fetch_response = client
        .get(format!("/achievements/{}", achievement.id))
        .dispatch()
        .await;
    assert_eq!(fetch_response.status(), Status::NotFound);

    cleanup(&pool, user_id, saving_goal_id, None).await;
}
