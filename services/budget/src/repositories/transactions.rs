use crate::domains::{
    errors::Result,
    transactions::{
        report::PeriodFilter, CreateTransaction, MonthReference, Transaction, TransactionCategory,
        TransactionRecurrency, TransactionStatus, TransactionType, UpdateTransaction,
    },
};

use super::SqlxRepository;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TransactionRepository {
    async fn create_transaction(&self, transaction: CreateTransaction) -> Result<Transaction>;
    async fn list_transactions(&self) -> Result<Vec<Transaction>>;
    async fn get_transaction_by_id(&self, transaction_id: Uuid) -> Result<Option<Transaction>>;
    async fn delete_transaction_by_id(&self, transaction_id: Uuid) -> Result<Option<Transaction>>;
    async fn update_transaction_by_id(
        &self,
        transaction: Transaction,
        payload: UpdateTransaction,
    ) -> Result<Option<Transaction>>;
    async fn update_status(
        &self,
        transaction_id: Uuid,
        status: TransactionStatus,
    ) -> Result<Option<Transaction>>;
    async fn list_transactions_by_period(&self, period: &PeriodFilter) -> Result<Vec<Transaction>>;
}

#[async_trait::async_trait]
impl TransactionRepository for SqlxRepository {
    async fn list_transactions(&self) -> Result<Vec<Transaction>> {
        let transactions = sqlx::query_as!(
            Transaction,
            r#"
            SELECT
                transaction_id, 
                movement_type as "movement_type!: TransactionType",
                description, 
                amount, 
                due_date, 
                category as "category: TransactionCategory", 
                account_id, 
                recurring, 
                recurrence_frequency as "recurrence_frequency: TransactionRecurrency", 
                status as "status: TransactionStatus", 
                note, 
                month_reference as "month_reference!: MonthReference",
                year_reference,
                installment_number,
                created_at, 
                updated_at, 
                deleted_at
            FROM TRANSACTIONS
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(transactions)
    }

    async fn create_transaction(&self, transaction: CreateTransaction) -> Result<Transaction> {
        let transaction = sqlx::query_as!(
            Transaction,
            r#"
            INSERT INTO TRANSACTIONS (
                transaction_id,
                movement_type,
                description,
                amount,
                due_date,
                category,
                account_id,
                recurring,
                recurrence_frequency,
                month_reference,
                year_reference,
                status,
                installment_number
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13
            ) RETURNING 
                transaction_id, 
                movement_type as "movement_type!: TransactionType",
                description, 
                amount, 
                due_date, 
                category as "category: TransactionCategory", 
                account_id, 
                recurring, 
                recurrence_frequency as "recurrence_frequency: TransactionRecurrency", 
                status as "status: TransactionStatus", 
                note, 
                month_reference as "month_reference!: MonthReference",
                year_reference,
                installment_number,
                created_at, 
                updated_at, 
                deleted_at
            "#,
            Uuid::new_v4(),
            transaction.movement_type as TransactionType,
            transaction.description,
            transaction.amount,
            transaction.due_date,
            transaction.category as TransactionCategory,
            transaction.account_id,
            transaction.recurring,
            transaction.get_transaction_recurrence() as TransactionRecurrency,
            transaction.month_reference as MonthReference,
            transaction.year_reference,
            transaction.status as TransactionStatus,
            transaction.installment_number
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(transaction)
    }

    async fn get_transaction_by_id(&self, transaction_id: Uuid) -> Result<Option<Transaction>> {
        let transaction = sqlx::query_as!(
            Transaction,
            r#"
            SELECT
                transaction_id, 
                movement_type as "movement_type!: TransactionType",
                description, 
                amount, 
                due_date, 
                category as "category: TransactionCategory", 
                account_id, 
                recurring, 
                recurrence_frequency as "recurrence_frequency: TransactionRecurrency", 
                status as "status: TransactionStatus", 
                note, 
                month_reference as "month_reference!: MonthReference",
                year_reference,
                installment_number,
                created_at, 
                updated_at, 
                deleted_at
            FROM TRANSACTIONS
            WHERE transaction_id = $1
            "#,
            transaction_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(transaction)
    }

    async fn delete_transaction_by_id(&self, transaction_id: Uuid) -> Result<Option<Transaction>> {
        let transaction = sqlx::query_as!(
            Transaction,
            r#"
            UPDATE TRANSACTIONS SET
                updated_at = now(),
                deleted_at = now()
            WHERE
                transaction_id = $1
                AND deleted_at is null
            RETURNING 
                transaction_id, 
                movement_type as "movement_type!: TransactionType",
                description, 
                amount, 
                due_date, 
                category as "category: TransactionCategory", 
                account_id, 
                recurring, 
                recurrence_frequency as "recurrence_frequency: TransactionRecurrency", 
                status as "status: TransactionStatus", 
                note, 
                month_reference as "month_reference!: MonthReference",
                year_reference,
                installment_number,
                created_at, 
                updated_at, 
                deleted_at
            "#,
            transaction_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(transaction)
    }

    async fn update_transaction_by_id(
        &self,
        transaction: Transaction,
        payload: UpdateTransaction,
    ) -> Result<Option<Transaction>> {
        let transaction = sqlx::query_as!(
            Transaction,
            r#"
            UPDATE TRANSACTIONS SET
                updated_at = now()
            WHERE 
                transaction_id = $1
            RETURNING
                transaction_id, 
                movement_type as "movement_type!: TransactionType",
                description, 
                amount, 
                due_date, 
                category as "category: TransactionCategory", 
                account_id, 
                recurring, 
                recurrence_frequency as "recurrence_frequency: TransactionRecurrency", 
                status as "status: TransactionStatus", 
                note, 
                month_reference as "month_reference!: MonthReference",
                year_reference,
                installment_number,
                created_at, 
                updated_at, 
                deleted_at
            "#,
            transaction.transaction_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(transaction)
    }

    async fn update_status(
        &self,
        transaction_id: Uuid,
        status: TransactionStatus,
    ) -> Result<Option<Transaction>> {
        let transaction = sqlx::query_as!(
            Transaction,
            r#"
            UPDATE transactions SET status = $2 WHERE transaction_id = $1
            RETURNING
                transaction_id, 
                movement_type as "movement_type!: TransactionType",
                description, 
                amount, 
                due_date, 
                category as "category: TransactionCategory", 
                account_id, 
                recurring, 
                recurrence_frequency as "recurrence_frequency: TransactionRecurrency", 
                status as "status: TransactionStatus", 
                note, 
                month_reference as "month_reference!: MonthReference",
                year_reference,
                installment_number,
                created_at, 
                updated_at, 
                deleted_at

            "#,
            transaction_id,
            status as TransactionStatus
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(transaction)
    }

    async fn list_transactions_by_period(&self, period: &PeriodFilter) -> Result<Vec<Transaction>> {
        let transactions = sqlx::query_as!(
            Transaction,
            r#"
            SELECT
                transaction_id, 
                movement_type as "movement_type!: TransactionType",
                description, 
                amount, 
                due_date, 
                category as "category: TransactionCategory", 
                account_id, 
                recurring, 
                recurrence_frequency as "recurrence_frequency: TransactionRecurrency", 
                installment_number,
                status as "status: TransactionStatus", 
                note, 
                month_reference as "month_reference!: MonthReference",
                year_reference,
                created_at, 
                updated_at, 
                deleted_at
            FROM transactions
            WHERE
                month_reference = $1 AND year_reference = $2
            "#,
            period.transform_month() as MonthReference,
            period.year
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(transactions)
    }
}
