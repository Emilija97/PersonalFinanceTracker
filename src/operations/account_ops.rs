use crate::models::account::Account;
use crate::{dtos::account_dtos::AccountInDTO, enums::custom_enums::AccountType};
use sqlx::{Error, PgPool, Row};
use uuid::Uuid;

pub async fn get_all_accounts(pool: &PgPool) -> Result<Vec<Account>, Error> {
    let transactions = sqlx::query_as::<_, Account>(
        r#"
        SELECT * FROM accounts
    "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(transactions)
}

pub async fn find_account_by_id(pool: &PgPool, account_id: Uuid) -> Result<Option<Account>, Error> {
    let row = sqlx::query("SELECT *, account_type::TEXT FROM accounts WHERE id = $1")
        .bind(account_id)
        .fetch_optional(pool)
        .await?;

    if let Some(row) = row {
        let account = Account {
            id: row.get("id"),
            name: row.get("name"),
            account_type: AccountType::from_str(row.get::<&str, _>("account_type"))
                .unwrap_or(AccountType::Bank),
            balance: row.get("balance"),
            user_id: row.get("user_id"),
        };
        Ok(Some(account))
    } else {
        Ok(None)
    }
}

pub async fn create_account(pool: &PgPool, account_dto: &AccountInDTO) -> Result<Account, Error> {
    let account = sqlx::query_as::<_, Account>(
        r#"
        INSERT INTO accounts (name, account_type, balance, user_id)
        VALUES ($1, $2, $3, $4)
        RETURNING *
    "#,
    )
    .bind(&account_dto.name)
    .bind(&account_dto.account_type)
    .bind(account_dto.balance)
    .bind(&account_dto.user_id)
    .fetch_one(pool)
    .await?;

    Ok(account)
}

pub async fn update_account(
    pool: &PgPool,
    account_id: Uuid,
    account_dto: &AccountInDTO,
) -> Result<Account, Error> {
    let account = sqlx::query_as::<_, Account>(
        r#"
        UPDATE accounts
        SET name = $1, account_type = $2, balance = $3
        WHERE id = $4
        RETURNING id, name, account_type, balance, user_id
        "#,
    )
    .bind(&account_dto.name)
    .bind(&account_dto.account_type)
    .bind(&account_dto.balance)
    .bind(account_id)
    .fetch_one(pool)
    .await?;

    Ok(account)
}

pub async fn delete_account(pool: &PgPool, account_id: Uuid) -> Result<(), Error> {
    sqlx::query(
        r#"
        DELETE FROM accounts WHERE id = $1"#,
    )
    .bind(account_id)
    .execute(pool)
    .await?;

    Ok(())
}
