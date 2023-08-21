use crate::{dtos::category_dtos::CategoryInDTO, models::categories::Category};
use sqlx::{postgres::PgPool, Error};
use uuid::Uuid;

pub async fn find_category_by_id(
    pool: &PgPool,
    category_id: Uuid,
) -> Result<Option<Category>, Error> {
    let category = sqlx::query_as!(
        Category,
        "SELECT * FROM categories WHERE id = $1",
        category_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(category)
}

pub async fn fetch_all_categories(pool: &PgPool) -> Result<Vec<Category>, sqlx::Error> {
    let categories = sqlx::query_as::<_, Category>(r#"SELECT * FROM categories"#)
        .fetch_all(pool)
        .await?;

    Ok(categories)
}

pub async fn create_category(
    pool: &PgPool,
    category_dto: &CategoryInDTO,
) -> Result<Category, sqlx::Error> {
    let category = sqlx::query_as::<_, Category>(
        r#"
        INSERT INTO categories (name, user_id)
        VALUES ($1, $2)
        RETURNING *
    "#,
    )
    .bind(&category_dto.name)
    .bind(&category_dto.user_id)
    .fetch_one(pool)
    .await?;

    Ok(category)
}

pub async fn update_category_in_db(
    pool: &PgPool,
    category_id: Uuid,
    category_dto: &CategoryInDTO,
) -> Result<Category, sqlx::Error> {
    let category = sqlx::query_as::<_, Category>(
        r#"
        UPDATE categories SET name = $1 WHERE id = $2 RETURNING *
    "#,
    )
    .bind(&category_dto.name)
    .bind(category_id)
    .fetch_one(pool)
    .await?;

    Ok(category)
}

pub async fn delete_category(pool: &PgPool, category_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM categories
        WHERE id = $1
    "#,
    )
    .bind(category_id)
    .execute(pool)
    .await?;

    Ok(())
}
