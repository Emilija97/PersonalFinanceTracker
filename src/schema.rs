// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "account_type"))]
    pub struct AccountType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "transaction_type"))]
    pub struct TransactionType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AccountType;

    accounts (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        account_type -> AccountType,
        balance -> Float8,
        user_id -> Uuid,
    }
}

diesel::table! {
    achievements (id) {
        id -> Uuid,
        date_achieved -> Timestamp,
        amount_saved -> Float8,
        goal_id -> Uuid,
    }
}

diesel::table! {
    budgets (id) {
        id -> Uuid,
        name -> Text,
        amount -> Float8,
        start_date -> Timestamp,
        end_date -> Timestamp,
        user_id -> Uuid,
        category_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    categories (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        user_id -> Uuid,
    }
}

diesel::table! {
    saving_goals (id) {
        id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        target_amount -> Float8,
        current_amount -> Float8,
        deadline -> Date,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TransactionType;

    transactions (id) {
        id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        amount -> Float8,
        date -> Timestamp,
        category_id -> Uuid,
        transaction_type -> TransactionType,
        user_id -> Uuid,
        account_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(accounts -> users (user_id));
diesel::joinable!(achievements -> saving_goals (goal_id));
diesel::joinable!(budgets -> categories (category_id));
diesel::joinable!(budgets -> users (user_id));
diesel::joinable!(categories -> users (user_id));
diesel::joinable!(saving_goals -> users (user_id));
diesel::joinable!(transactions -> accounts (account_id));
diesel::joinable!(transactions -> categories (category_id));
diesel::joinable!(transactions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    achievements,
    budgets,
    categories,
    saving_goals,
    transactions,
    users,
);
