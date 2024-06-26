use sqlx::PgPool;
pub mod accounts;
pub mod installments;
pub mod transactions;
pub mod recurrence_transactions;

#[derive(Clone)]
pub struct SqlxRepository {
    pool: PgPool,
}

impl SqlxRepository {
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
