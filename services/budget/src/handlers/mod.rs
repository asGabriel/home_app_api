use std::sync::Arc;

use crate::repositories::{
    accounts::AccountRepository, installments::InstallmentRepository,
    transactions::TransactionRepository,
};

pub mod accounts;
pub mod installments;
pub mod transactions;

#[derive(Clone)]
pub struct Handler {
    transaction_repository: Arc<dyn TransactionRepository + Send + Sync>,
    account_repository: Arc<dyn AccountRepository + Send + Sync>,
    report_repository: Arc<dyn TransactionRepository + Send + Sync>,
    installment_repository: Arc<dyn InstallmentRepository + Send + Sync>,
}

impl Handler {
    pub const fn new(
        transactions_repository: Arc<dyn TransactionRepository + Send + Sync>,
        accounts_repository: Arc<dyn AccountRepository + Send + Sync>,
        report_repository: Arc<dyn TransactionRepository + Send + Sync>,
        installment_repository: Arc<dyn InstallmentRepository + Send + Sync>,
    ) -> Self {
        Self {
            transaction_repository: transactions_repository,
            account_repository: accounts_repository,
            report_repository: report_repository,
            installment_repository: installment_repository,
        }
    }
}
