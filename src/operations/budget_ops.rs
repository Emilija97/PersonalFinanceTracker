use crate::{dtos::budget_dtos::BudgetInDTO, models::budget::Budget};
use sqlx::{postgres::PgPool, Error};
use uuid::Uuid;

pub async fn find_budget_by_id(pool: &PgPool, budget_id: Uuid) -> Result<Option<Budget>, Error> {
    let budget = sqlx::query_as!(Budget, "SELECT * FROM budgets WHERE id = $1", budget_id)
        .fetch_optional(pool)
        .await?;
    Ok(budget)
}

pub async fn fetch_all_budgets(pool: &PgPool) -> Result<Vec<Budget>, sqlx::Error> {
    let budgets = sqlx::query_as::<_, Budget>(r#"SELECT * FROM budgets"#)
        .fetch_all(pool)
        .await?;

    Ok(budgets)
}

pub async fn create_budget(pool: &PgPool, budget_dto: &BudgetInDTO) -> Result<Budget, sqlx::Error> {
    let row = sqlx::query_as::<_, Budget>(
        r#"
        INSERT INTO budgets (name, amount, start_date, end_date, user_id, category_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
    )
    .bind(&budget_dto.name)
    .bind(&budget_dto.amount)
    .bind(&budget_dto.start_date)
    .bind(&budget_dto.end_date)
    .bind(&budget_dto.user_id)
    .bind(&budget_dto.category_id) // This is an Option<Uuid>, so it's okay if it's None
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn update_budget_in_db(
    pool: &PgPool,
    budget_id: Uuid,
    budget_dto: &BudgetInDTO,
) -> Result<Budget, sqlx::Error> {
    let row = sqlx::query_as::<_, Budget>(
        r#"
        UPDATE budgets
        SET name = $1, amount = $2, start_date = $3, end_date = $4, user_id = $5, category_id = $6
        WHERE id = $7
        RETURNING *
        "#,
    )
    .bind(&budget_dto.name)
    .bind(&budget_dto.amount)
    .bind(&budget_dto.start_date)
    .bind(&budget_dto.end_date)
    .bind(&budget_dto.user_id)
    .bind(&budget_dto.category_id)
    .bind(budget_id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn delete_budget(pool: &PgPool, budget_id: Uuid) -> Result<(), sqlx::Error> {
    // let rows_deleted = sqlx::query("DELETE FROM budgets WHERE id = $1")
    //     .bind(budget_id)
    //     .execute(pool)
    //     .await?
    //     .rows_affected();

    // Ok(rows_deleted)
    sqlx::query(
        r#"
        DELETE FROM budgets
        WHERE id = $1
    "#,
    )
    .bind(budget_id)
    .execute(pool)
    .await?;

    Ok(())
}
