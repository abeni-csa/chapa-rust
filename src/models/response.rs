//! Response model for chapa API

use serde::Deserialize;
use serde_json::Value;

use crate::models::{
    bank::Bank,
    payment::{CheckoutURL, SubaccountData, VerifyData},
    transaction::{GetTransactionsData, TransactionEventData},
    transfer::{BulkTransferData, MetaPagination, TransferData},
};

/// Represents a generic response from the Chapa API.
#[derive(Debug, Clone, Deserialize)]
pub struct ChapaResponse<T> {
    /// The status message of the response.
    pub message: Value, // NOTE: Changed to Value to handle empty strings or other types, since some responses might return non-string messages
    #[serde(default = "unspecified_status")]
    /// The status of the response.
    pub status: String,
    /// The data section of the response.
    pub data: T,
}

/// Datat Structe GetTransactionsResponse , which t allows you to view all the transactions the Chapa API
#[derive(Debug, Deserialize)]
pub struct ChapaTransferListResponse {
    /// The status message of the response.
    pub message: String, // FIX: Changed to Value to handle empty strings or other types, since some responses might return non-string messages
    #[serde(default = "unspecified_status")]
    /// The status of the response.
    pub status: String,
    /// Pagination details for a list of transactions.
    #[serde(alias = "pagination")]
    pub meta: Option<MetaPagination>, // often returned at top level
    /// The data section of the response.
    pub data: Option<Vec<TransferData>>, // array of transfers
}

fn unspecified_status() -> String {
    "Unspecified".to_string()
}

/// Type alias for GetBanksResponse, which contains a list of banks.
pub type GetBanksResponse = ChapaResponse<Option<Vec<Bank>>>;
/// Type alias for InitializeResponse, which contains the checkout URL.
pub type InitializeResponse = ChapaResponse<Option<CheckoutURL>>;
/// Type alias for VerifyResponse, which contains the verification data.
pub type VerifyResponse = ChapaResponse<Option<VerifyData>>;
/// Type alias for SubaccountResponse, which contains the subaccount data.
pub type SubaccountResponse = ChapaResponse<SubaccountData>;
/// Type alias for TransactionEventsResponse, which allows you to view the timeline for a transaction
pub type TransactionEventsResponse = ChapaResponse<Option<Vec<TransactionEventData>>>;
/// Type alias for GetTransactionsResponse , which t allows you to view all the transactions
pub type GetAllTransfersResponse = ChapaTransferListResponse;
/// Type alias for All Transaction respose wich conatine  all the transactions
pub type GetAllTransactionResponse = ChapaResponse<Option<GetTransactionsData>>;
/// trnasfer
pub type TransferResponse = ChapaResponse<Option<TransferData>>;
/// bulk trasfer
pub type BulkTransferResponse = ChapaResponse<Option<BulkTransferData>>;
