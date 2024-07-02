use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use finance::{category::Category, frequency::Frequency};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurrenceTransaction {
    pub recurrence_transaction_id: Uuid,
    pub account_id: Uuid,
    pub description: String,
    pub amount: BigDecimal,
    pub frequency: Frequency,
    pub reference: i32,
    pub category: Category,
    pub is_active: bool,
    pub start_date: NaiveDate,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedTransaction {
    pub id: i32,
    pub recurrence_transaction_id: Uuid,
    pub transaction_id: Uuid,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecurrenceTransaction {
    pub account_id: Uuid,
    pub description: String,
    pub amount: BigDecimal,
    pub frequency: Frequency,
    pub reference: i32,
    pub category: Category,
    pub start_date: NaiveDate,
}
