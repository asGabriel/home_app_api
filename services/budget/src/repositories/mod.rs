use sqlx::PgPool;
pub mod accounts;
pub mod installments;
pub mod recurrences;
pub mod settlements;
pub mod transactions;
pub mod financial_plans;

#[derive(Clone)]
pub struct SqlxRepository {
    pool: PgPool,
}

impl SqlxRepository {
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
