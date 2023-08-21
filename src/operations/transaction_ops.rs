use crate::{dtos::transaction_dtos::TransactionInDTO, models::transactions::Transaction, enums::custom_enums::TransactionType};
use sqlx::{postgres::PgPool, Row};
use uuid::Uuid;

pub async fn fetch_all_transactions(pool: &PgPool) -> Result<Vec<Transaction>, sqlx::Error> {
    let transactions = sqlx::query_as::<_, Transaction>(
        r#"
        SELECT * FROM transactions
    "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(transactions)
}

pub async fn find_transaction_by_id(pool: &PgPool, transaction_id: Uuid) -> Result<Option<Transaction>, sqlx::Error> {
    let row = sqlx::query("SELECT *, transaction_type::TEXT FROM transactions WHERE id = $1")
        .bind(transaction_id)
        .fetch_optional(pool)
        .await?;

    if let Some(row) = row {
        let transaction = Transaction {
            id: row.get("id"),
            transaction_type: TransactionType::from_str(row.get::<&str, _>("transaction_type"))
                .unwrap_or(TransactionType::Income),
            user_id: row.get("user_id"),
            title: row.get("title"),
            amount: row.get("amount"),
            date: row.get("date"),
            category_id: row.get("category_id"),
            account_id: row.get("account_id"),
        };
        Ok(Some(transaction))
    } else {
        Ok(None)
    }
}


pub async fn create_transaction(
    pool: &PgPool,
    transaction_dto: &TransactionInDTO,
) -> Result<Transaction, sqlx::Error> {
    let transaction = sqlx::query_as::<_, Transaction>(
        r#"
        INSERT INTO transactions (title, amount, date, category_id, transaction_type, user_id, account_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
    "#,
    )
    .bind(&transaction_dto.title)
    .bind(transaction_dto.amount)
    .bind(transaction_dto.date) 
    .bind(&transaction_dto.category_id)
    .bind(&transaction_dto.transaction_type)
    .bind(&transaction_dto.user_id)
    .bind(&transaction_dto.account_id)
    .fetch_one(pool)
    .await?;

    Ok(transaction)
}

pub async fn update_transaction(
    pool: &PgPool,
    transaction_id: Uuid,
    transaction_dto: &TransactionInDTO,
) -> Result<Transaction, sqlx::Error> {
    let transaction = sqlx::query_as::<_, Transaction>(
        r#"
        UPDATE transactions
        SET title = $1, amount = $2, date = $3, category_id = $4, transaction_type = $5, user_id = $6, account_id = $7
        WHERE id = $8
        RETURNING *
    "#,
    )
    .bind(&transaction_dto.title)
    .bind(transaction_dto.amount)
    .bind(transaction_dto.date) 
    .bind(&transaction_dto.category_id)
    .bind(&transaction_dto.transaction_type)
    .bind(&transaction_dto.user_id)
    .bind(&transaction_dto.account_id)
    .bind(transaction_id)
    .fetch_one(pool)
    .await?;

    Ok(transaction)
}

pub async fn delete_transaction(pool: &PgPool, transaction_id: Uuid) -> Result<u64, sqlx::Error> {
    let deleted = sqlx::query(
        r#"
        DELETE FROM transactions
        WHERE id = $1
    "#,
    )
    .bind(transaction_id)
    .execute(pool)
    .await?;

    Ok(deleted.rows_affected())
}

