//! Models related to bank transfers.

use crate::models::bank::Currency;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents pagination details for a list of transactions.
#[derive(Debug, Serialize, Deserialize)]
pub struct MetaPagination {
    /// Page number of the current set of transactions.
    pub current_page: u32,
    /// URL to the first page of transactions.
    pub first_page_url: String,
    /// Page number of the last set of transactions.
    pub last_page: u32,
    /// URL to the last page of transactions.
    pub last_page_url: Option<String>,
    /// URL to the next page page of transactions.
    pub next_page_url: Option<String>,
    /// URL to the this current page of transactions.
    pub path: Option<String>,
    /// How many transactions are in a single page.
    pub per_page: u32,
    /// URL to the previous page of transactions.
    pub prev_page_url: Option<String>,
    /// To
    pub to: u32,
    /// Total
    pub total: u32,
    /// error
    pub error: Vec<String>,
}

/// Represents the options required to initiate a bank transfer.
#[derive(Debug, Serialize, Deserialize)]
pub struct InitateTransferOptions {
    /// The name of the account holder.
    pub account_name: Option<String>,
    /// The bank account number to which the transfer will be made.
    pub account_number: String,
    /// The amount to be transferred.
    pub amount: String,
    /// ETB or USD
    pub currency: Option<Currency>,
    /// A unique reference for the transfer.
    pub reference: Option<String>,
    /// The bank code of the recipient's bank.
    pub bank_code: u32,
}

/// Represents the detailed data received when bulk transaction created .
#[derive(Debug, Deserialize)]
pub struct TransferData {
    /// The name of the account holder.
    pub account_name: String,
    /// The bank account number to which the transfer will be made.
    pub account_number: String,
    /// ETB or USD
    pub currency: Currency,
    /// AMOUT
    pub amount: u32,
    /// 100,
    pub charge: u32, // 0,
    /// trasfer method
    pub transfer_type: String, // "bank",
    /// chapa transfer id
    pub chapa_reference: String, // "4d6a7cb7-0d51-4c27-9a19-cc3f066c85a3",
    /// bank code form chapa
    pub bank_code: u32, // 128,
    /// bank name
    pub bank_name: String, // "Bunna Bank",
    /// cross partiy reffecnce
    pub bank_reference: Option<String>, // null,
    /// stuats of the transactions
    pub status: String, // "success",
    /// transaction refectc id
    pub reference: Option<String>, // "chewatatest-6669",
    /// The timestamp when the transaction was created.
    pub created_at: DateTime<Utc>,
    /// The timestamp when the transaction was last updated.
    pub updated_at: DateTime<Utc>,
}
/// bulk trasfer options
#[derive(Debug, Serialize)]
pub struct BulkTransferOptions {
    /// The title to be displayed on the payment interface.
    pub title: String,
    /// The currency for the transaction (e.g., "ETB", "USD").
    pub currency: Currency,
    /// The list of Bulk Data transfer
    pub bulk_data: Vec<InitateTransferOptions>,
}
/// dtat type for bulk data transfer
#[derive(Debug, Deserialize)]
pub struct BulkTransferData {
    /// id of the bulk truafer
    pub id: u64,
    /// timespam for when the trasaction is created
    pub created_at: DateTime<Utc>,
}
