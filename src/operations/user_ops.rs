use chrono::Local;
// use crate::dtos::user_dtos::UserInDTO;
// use crate::models::user::User;
use sqlx::{postgres::PgPool, Error};
use uuid::Uuid;

use crate::{dtos::user_dtos::UserInDTO, models::user::User};

pub async fn find_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>, Error> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_optional(pool)
        .await?;
    Ok(user)
}

pub async fn fetch_all_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as::<_, User>(
        r#"
        SELECT id, username, email, created_at, updated_at 
        FROM users
    "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

pub async fn create_user(pool: &PgPool, user_dto: &UserInDTO) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (username, email)
        VALUES ($1, $2)
        RETURNING id, username, email, created_at, updated_at
    "#,
    )
    .bind(&user_dto.username)
    .bind(&user_dto.email)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn update_user_in_db(
    pool: &PgPool,
    user_id: Uuid,
    user_dto: &UserInDTO,
) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET username = $1, email = $2, updated_at = $3
        WHERE id = $4
        RETURNING id, username, email, created_at, updated_at
    "#,
    )
    .bind(&user_dto.username)
    .bind(&user_dto.email)
    .bind(Local::now().naive_local())
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn delete_user(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM users
        WHERE id = $1
    "#,
    )
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}
