use crate::{dtos::saving_goal_dtos::SavingGoalInDTO, models::saving_goals::SavingGoal};
use chrono::Local;
use sqlx::{postgres::PgPool, Error};
use uuid::Uuid;

pub async fn find_saving_goal_by_id(
    pool: &PgPool,
    saving_goal_id: Uuid,
) -> Result<Option<SavingGoal>, Error> {
    let saving_goal = sqlx::query_as!(
        SavingGoal,
        "SELECT * FROM saving_goals WHERE id = $1",
        saving_goal_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(saving_goal)
}

pub async fn fetch_all_saving_goals(pool: &PgPool) -> Result<Vec<SavingGoal>, sqlx::Error> {
    let saving_goals = sqlx::query_as::<_, SavingGoal>(r#"SELECT * FROM saving_goals"#)
        .fetch_all(pool)
        .await?;

    Ok(saving_goals)
}

pub async fn create_saving_goal(
    pool: &PgPool,
    saving_goal_dto: &SavingGoalInDTO,
) -> Result<SavingGoal, sqlx::Error> {
    let row = sqlx::query_as::<_, SavingGoal>(
        r#"
        INSERT INTO saving_goals (title, target_amount, current_amount, deadline, user_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
    )
    .bind(&saving_goal_dto.title)
    .bind(&saving_goal_dto.target_amount)
    .bind(&saving_goal_dto.current_amount)
    .bind(&saving_goal_dto.deadline)
    .bind(&saving_goal_dto.user_id)
    .bind(Local::now().naive_local())
    .bind(Local::now().naive_local())
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn update_saving_goal_in_db(
    pool: &PgPool,
    saving_goal_id: Uuid,
    saving_goal_dto: &SavingGoalInDTO,
) -> Result<SavingGoal, sqlx::Error> {
    let row = sqlx::query_as::<_, SavingGoal>(
        r#"
        UPDATE saving_goals
        SET title = $1, target_amount = $2, current_amount = $3, deadline = $4, user_id = $5, updated_at = $6
        WHERE id = $7
        RETURNING *
        "#,
    )
    .bind(&saving_goal_dto.title)
    .bind(&saving_goal_dto.target_amount)
    .bind(&saving_goal_dto.current_amount)
    .bind(&saving_goal_dto.deadline)
    .bind(&saving_goal_dto.user_id)
    .bind(Local::now().naive_local())
    .bind(saving_goal_id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn delete_saving_goal(pool: &PgPool, saving_goal_id: Uuid) -> Result<(), sqlx::Error> {
    // let rows_deleted = sqlx::query("DELETE FROM saving_goals WHERE id = $1")
    //     .bind(saving_goal_id)
    //     .execute(pool)
    //     .await?
    //     .rows_affected();

    // Ok(rows_deleted)
    sqlx::query(
        r#"
        DELETE FROM saving_goals
        WHERE id = $1
    "#,
    )
    .bind(saving_goal_id)
    .execute(pool)
    .await?;

    Ok(())
}
