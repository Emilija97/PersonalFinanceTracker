use crate::{dtos::achievement_dtos::AchievementInDTO, models::achievement::Achievement};
use sqlx::{postgres::PgPool, Error};
use uuid::Uuid;

pub async fn find_achievement_by_id(
    pool: &PgPool,
    achievement_id: Uuid,
) -> Result<Option<Achievement>, Error> {
    let achievement = sqlx::query_as!(
        Achievement,
        "SELECT * FROM achievements WHERE id = $1",
        achievement_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(achievement)
}

pub async fn fetch_all_achievements(pool: &PgPool) -> Result<Vec<Achievement>, sqlx::Error> {
    let achievements = sqlx::query_as::<_, Achievement>(r#"SELECT * FROM achievements"#)
        .fetch_all(pool)
        .await?;

    Ok(achievements)
}

pub async fn create_achievement(
    pool: &PgPool,
    achievement_dto: &AchievementInDTO,
) -> Result<Achievement, sqlx::Error> {
    let row = sqlx::query_as::<_, Achievement>(
        r#"
        INSERT INTO achievements (goal_id, date_achieved, amount_saved)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(&achievement_dto.goal_id)
    .bind(&achievement_dto.date_achieved)
    .bind(&achievement_dto.amount_saved)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn update_achievement_in_db(
    pool: &PgPool,
    achievement_id: Uuid,
    achievement_dto: &AchievementInDTO,
) -> Result<Achievement, sqlx::Error> {
    let row = sqlx::query_as::<_, Achievement>(
        r#"
        UPDATE achievements
        SET goal_id = $1, date_achieved = $2, amount_saved = $3
        WHERE id = $4
        RETURNING *
        "#,
    )
    .bind(&achievement_dto.goal_id)
    .bind(&achievement_dto.date_achieved)
    .bind(&achievement_dto.amount_saved)
    .bind(achievement_id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn delete_achievement(pool: &PgPool, achievement_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM achievements
        WHERE id = $1
    "#,
    )
    .bind(achievement_id)
    .execute(pool)
    .await?;

    Ok(())
}
